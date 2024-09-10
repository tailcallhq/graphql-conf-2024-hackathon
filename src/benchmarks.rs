use std::{path::PathBuf, sync::LazyLock};

use anyhow::{anyhow, Context, Result};
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

#[instrument(skip_all)]
pub async fn run_benchmarks(project: &str) -> Result<()> {
    info!("Starting benchmark");
    let mut mock_path = PathBuf::from(ROOT_DIR);
    mock_path.push("benchmark.sh");

    let mut output_path = PathBuf::from(ROOT_DIR);
    output_path.push("results");
    output_path.push(&project);

    fs::create_dir_all(&output_path).await?;

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

        let output_path = output_path.join(format!("{bench_name}.out"));

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)
            .await?;

        file.write_all(&output.stdout).await?;
    }

    Ok(())
}
