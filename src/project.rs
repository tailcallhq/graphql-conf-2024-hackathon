use anyhow::{anyhow, Result};
use easy_retry::EasyRetry;
use std::{path::PathBuf, time::Duration};
use tokio::{fs, io::AsyncWriteExt};
use tracing::{info, instrument};

use crate::{
    command::{Command, CommandInstance},
    graphql_tests::run_graphql_tests,
    request::graphql_request,
    utils::env_default,
    ROOT_DIR,
};

/// Runs tests and benchmarks for single project
pub struct Project {
    path: PathBuf,
    name: String,
}

impl Project {
    pub fn new(path: PathBuf) -> Result<Self> {
        let name = path
            .file_name()
            .ok_or(anyhow!("Expected directory inside 'projects'"))?
            .to_string_lossy()
            .into_owned();

        Ok(Project { path, name })
    }

    /// Run the tests and benchmarks
    #[instrument(skip_all, fields(project = &self.name))]
    pub async fn run_project(self) -> Result<()> {
        info!("Starting project: {}", &self.name);

        let mock_server = self.run_mock_server().await?;
        let server = self.run_server().await?;

        run_graphql_tests().await?;
        self.run_benchmark().await?;
        run_graphql_tests().await?;

        mock_server.kill().await?;
        info!("Kill the server process");
        server.kill().await?;

        Ok(())
    }

    #[instrument(skip_all)]
    async fn run_mock_server(&self) -> Result<CommandInstance> {
        info!("Starting mock server");

        let mut mock_path = PathBuf::from(ROOT_DIR);
        mock_path.push("mock.sh");
        let mut command = Command::from_path(&mock_path)?;

        command.args(&["mocks/1.json"]);
        let command = command.run()?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        Ok(command)
    }

    #[instrument(skip_all)]
    async fn run_server(&self) -> Result<CommandInstance> {
        info!("Run run.sh");
        let mut run_path = self.path.clone();
        run_path.push("run.sh");

        let mut command = Command::from_path(&run_path)?;
        let command = command.run()?;

        let retry = EasyRetry::new_linear_async(
            env_default("RUN_SCRIPT_RETRY_TIMEOUT", 100),
            env_default("RUN_SCRIPT_RETRY_ATTEMPTS", 10),
        );

        // wait until the server is ready for responses
        retry
            .run_async(|| async {
                info!("Attempting to request the server");

                let result = graphql_request(
                    "
                query {
                    user(id: 1) {
                        name
                    }
                }
            ",
                )
                .await;

                if result.is_err() {
                    info!("Failed to resolve the response");
                } else {
                    info!("Request to server successful");
                }

                result
            })
            .await?;

        Ok(command)
    }

    #[instrument(skip_all)]
    async fn run_benchmark(&self) -> Result<()> {
        info!("Starting benchmark");
        let mut mock_path = PathBuf::from(ROOT_DIR);
        mock_path.push("benchmark.sh");
        let mut command = Command::from_path(&mock_path)?;

        command.args(&["1"]);

        let output = command.run_and_capture().await?;

        info!(
            "Benchmark results:\n\n {}",
            String::from_utf8_lossy(&output.stdout)
        );

        let mut output_path = PathBuf::from(ROOT_DIR);
        output_path.push("results");
        output_path.push(&self.name);

        fs::create_dir_all(&output_path).await?;

        output_path.push("bench_1.out");

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)
            .await?;

        file.write_all(&output.stdout).await?;

        Ok(())
    }
}
