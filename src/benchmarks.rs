use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncWriteExt};
use tracing::{info, instrument};

use crate::{command::Command, ROOT_DIR};

#[derive(Serialize, Deserialize, Default)]
struct Stats {
    connect_errors: u64,
    read_errors: u64,
    write_errors: u64,
    timeout_errors: u64,
    rps: u64,
}

static BENCHES: LazyLock<Result<Vec<String>>> = LazyLock::new(|| {
    let tests_path = format!("{ROOT_DIR}/benches");
    let mut tests = Vec::new();

    for entry in std::fs::read_dir(tests_path)? {
        let path = entry?.path();

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext == "lua" {
                let name = path
                    .file_stem()
                    .map(|s| s.to_string_lossy())
                    .context("expected the name")?;
                tests.push(name.to_string());
            }
        }
    }

    Ok(tests)
});

#[derive(Serialize, Deserialize, Default)]
struct AllStats(BTreeMap<String, Stats>);

impl Deref for AllStats {
    type Target = BTreeMap<String, Stats>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AllStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AllStats {
    fn score(&self, baseline: &AllStats) -> Result<u64> {
        let mut sum: u64 = 0;

        for (key, stats) in &self.0 {
            let baseline_stats = baseline
                .get(key)
                .context("Cannot find specific key in baseline stats")?;
            sum += 1000 * stats.rps / baseline_stats.rps;
        }

        Ok(sum / self.len() as u64)
    }
}

#[instrument(skip_all)]
pub async fn run_benchmarks(output_path: &Path) -> Result<()> {
    info!("Starting benchmark");
    let mut mock_path = PathBuf::from(ROOT_DIR);
    mock_path.push("benchmark.sh");

    fs::create_dir_all(&output_path).await?;

    let baseline_stats: AllStats = serde_json::from_str(
        &fs::read_to_string(Path::new(ROOT_DIR).join("reference/results/stats.json")).await?,
    )?;

    let mut stats = AllStats::default();

    for bench_name in BENCHES
        .as_ref()
        .map_err(|e| anyhow!("Failed to resolve benches due to error: {e}"))?
    {
        info!("Run benchmark: `{bench_name}`");

        let mut command = Command::from_path(&mock_path)?;

        command.args(&[bench_name]);

        let output = command.run_and_capture().await?;

        info!(
            "Benchmark results:\n\n {}",
            String::from_utf8_lossy(&output.stdout)
        );

        let out_path = output_path.join(format!("{bench_name}.out"));

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(out_path)
            .await?;

        let stdout = &output.stdout;

        file.write_all(stdout).await?;

        let single_stats = parse_wrk(stdout).context("Failed to parse wrk output")?;

        check_errors(&single_stats)
            .context("Connection errors found during execution, check benchmark output")?;

        stats.insert(bench_name.to_string(), single_stats);
    }

    let json_path = output_path.join("stats.json");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(json_path)
        .await?;

    file.write_all(serde_json::to_string_pretty(&stats)?.as_bytes())
        .await?;

    let score = output_path.join("score.out");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(score)
        .await?;

    file.write_all(stats.score(&baseline_stats)?.to_string().as_bytes())
        .await?;

    Ok(())
}

fn parse_wrk(output: &Vec<u8>) -> Result<Stats> {
    let output_str = String::from_utf8_lossy(output);

    let (connect_errors, read_errors, write_errors, timeout_errors) = extract_errors(&output_str);

    let rps = parse_u64(&output_str, Regex::new(r"Requests\/sec:\s+(\d+).?\d+")?)
        .context("Failed to parse rps")?;

    Ok(Stats {
        connect_errors,
        read_errors,
        write_errors,
        timeout_errors,
        rps,
    })
}

fn parse_u64(data: &str, re: Regex) -> anyhow::Result<u64> {
    if let Some(caps) = re.captures(data) {
        let value = &caps[1];
        Ok(value.parse().context("Failed to parse u64")?)
    } else {
        bail!("Failed to parse {:?}", re)
    }
}

fn extract_errors(data: &str) -> (u64, u64, u64, u64) {
    let re = Regex::new(
        r"Socket\s+errors:\s+connect\s+(\d+),\s+read\s+(\d+).\s+write\s+(\d+).\s+timeout\s+(\d+)",
    )
    .unwrap();
    if let Some(caps) = re.captures(data) {
        (
            caps[1].parse().unwrap_or(0),
            caps[2].parse().unwrap_or(0),
            caps[3].parse().unwrap_or(0),
            caps[4].parse().unwrap_or(0),
        )
    } else {
        (0, 0, 0, 0)
    }
}

fn check_errors(single_stats: &Stats) -> anyhow::Result<()> {
    if single_stats.read_errors > 0 {
        bail!("Execution failed because read_errors exist")
    }

    if single_stats.write_errors > 0 {
        bail!("Execution failed because write_errors exist")
    }

    if single_stats.connect_errors > 0 {
        bail!("Execution failed because connect_errors exist")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    mod stats {
        use crate::benchmarks::{AllStats, Stats};

        #[test]
        fn test_score_example() {
            let mut stats = AllStats::default();

            stats.insert(
                "posts-title".to_owned(),
                Stats {
                    rps: 100,
                    ..Default::default()
                },
            );
            stats.insert(
                "posts-with-user".to_owned(),
                Stats {
                    rps: 40,
                    ..Default::default()
                },
            );
            stats.insert(
                "posts-nested".to_owned(),
                Stats {
                    rps: 60,
                    ..Default::default()
                },
            );

            let mut baseline = AllStats::default();

            baseline.insert(
                "posts-title".to_owned(),
                Stats {
                    rps: 50,
                    ..Default::default()
                },
            );
            baseline.insert(
                "posts-with-user".to_owned(),
                Stats {
                    rps: 50,
                    ..Default::default()
                },
            );
            baseline.insert(
                "posts-nested".to_owned(),
                Stats {
                    rps: 50,
                    ..Default::default()
                },
            );

            assert_eq!(stats.score(&baseline).unwrap(), 1333);
        }

        #[test]
        fn test_score_equal() {
            let mut stats = AllStats::default();

            stats.insert(
                "posts-title".to_owned(),
                Stats {
                    rps: 100,
                    ..Default::default()
                },
            );
            stats.insert(
                "posts-with-user".to_owned(),
                Stats {
                    rps: 60,
                    ..Default::default()
                },
            );
            stats.insert(
                "posts-nested".to_owned(),
                Stats {
                    rps: 40,
                    ..Default::default()
                },
            );

            let mut baseline = AllStats::default();

            baseline.insert(
                "posts-title".to_owned(),
                Stats {
                    rps: 100,
                    ..Default::default()
                },
            );
            baseline.insert(
                "posts-with-user".to_owned(),
                Stats {
                    rps: 60,
                    ..Default::default()
                },
            );
            baseline.insert(
                "posts-nested".to_owned(),
                Stats {
                    rps: 40,
                    ..Default::default()
                },
            );

            assert_eq!(stats.score(&baseline).unwrap(), 1000);
        }

        #[test]
        fn test_score_less() {
            let mut stats = AllStats::default();

            stats.insert(
                "posts-title".to_owned(),
                Stats {
                    rps: 20,
                    ..Default::default()
                },
            );
            stats.insert(
                "posts-with-user".to_owned(),
                Stats {
                    rps: 10,
                    ..Default::default()
                },
            );
            stats.insert(
                "posts-nested".to_owned(),
                Stats {
                    rps: 5,
                    ..Default::default()
                },
            );

            let mut baseline = AllStats::default();

            baseline.insert(
                "posts-title".to_owned(),
                Stats {
                    rps: 40,
                    ..Default::default()
                },
            );
            baseline.insert(
                "posts-with-user".to_owned(),
                Stats {
                    rps: 30,
                    ..Default::default()
                },
            );
            baseline.insert(
                "posts-nested".to_owned(),
                Stats {
                    rps: 20,
                    ..Default::default()
                },
            );

            assert_eq!(stats.score(&baseline).unwrap(), 361);
        }
    }
}
