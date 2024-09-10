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

#[derive(Serialize, Deserialize)]
struct Stats {
    rps: u64,
}

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

        let single_stats = parse_wrk(stdout)?;

        stats.insert(bench_name.to_string(), single_stats);
    }

    let json_path = output_path.join(format!("stats.json"));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(json_path)
        .await?;

    file.write_all(serde_json::to_string_pretty(&stats)?.as_bytes())
        .await?;

    let score = output_path.join(format!("score.out"));

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

    // only the integer part of rps
    let re = Regex::new(r"Requests/sec:\s+(\d+)")?;

    let rps = if let Some(caps) = re.captures(&output_str) {
        let rps = &caps[1];

        rps.parse()?
    } else {
        bail!("Failed to parse wrk output");
    };

    Ok(Stats { rps })
}

#[cfg(test)]
mod tests {
    mod stats {
        use crate::benchmarks::{AllStats, Stats};

        #[test]
        fn test_score_example() {
            let mut stats = AllStats::default();

            stats.insert("posts-title".to_owned(), Stats { rps: 100 });
            stats.insert("posts-with-user".to_owned(), Stats { rps: 40 });
            stats.insert("posts-nested".to_owned(), Stats { rps: 60 });

            let mut baseline = AllStats::default();

            baseline.insert("posts-title".to_owned(), Stats { rps: 50 });
            baseline.insert("posts-with-user".to_owned(), Stats { rps: 50 });
            baseline.insert("posts-nested".to_owned(), Stats { rps: 50 });

            assert_eq!(stats.score(&baseline).unwrap(), 1333);
        }

        #[test]
        fn test_score_equal() {
            let mut stats = AllStats::default();

            stats.insert("posts-title".to_owned(), Stats { rps: 100 });
            stats.insert("posts-with-user".to_owned(), Stats { rps: 60 });
            stats.insert("posts-nested".to_owned(), Stats { rps: 40 });

            let mut baseline = AllStats::default();

            baseline.insert("posts-title".to_owned(), Stats { rps: 100 });
            baseline.insert("posts-with-user".to_owned(), Stats { rps: 60 });
            baseline.insert("posts-nested".to_owned(), Stats { rps: 40 });

            assert_eq!(stats.score(&baseline).unwrap(), 1000);
        }

        #[test]
        fn test_score_less() {
            let mut stats = AllStats::default();

            stats.insert("posts-title".to_owned(), Stats { rps: 20 });
            stats.insert("posts-with-user".to_owned(), Stats { rps: 10 });
            stats.insert("posts-nested".to_owned(), Stats { rps: 5 });

            let mut baseline = AllStats::default();

            baseline.insert("posts-title".to_owned(), Stats { rps: 40 });
            baseline.insert("posts-with-user".to_owned(), Stats { rps: 30 });
            baseline.insert("posts-nested".to_owned(), Stats { rps: 20 });

            assert_eq!(stats.score(&baseline).unwrap(), 361);
        }
    }
}
