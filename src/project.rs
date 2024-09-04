use anyhow::{anyhow, Result};
use std::{path::PathBuf, time::Duration};
use tracing::{info, instrument};

use crate::{
    command::{Command, CommandInstance},
    graphql_tests::run_graphql_tests,
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

        self.run_setup().await?;
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
    async fn run_setup(&self) -> Result<()> {
        info!("Run setup.sh");
        let mut setup_path = self.path.clone();
        setup_path.push("setup.sh");

        let mut command = Command::from_path(&setup_path)?;
        let mut command = command.run()?;

        command.wait().await?;

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

        // TODO: wait for server with timeout instead of explicit timeout
        tokio::time::sleep(Duration::from_secs(2)).await;

        Ok(command)
    }

    #[instrument(skip_all)]
    async fn run_benchmark(&self) -> Result<()> {
        info!("Starting benchmark");
        let mut mock_path = PathBuf::from(ROOT_DIR);
        mock_path.push("benchmark.sh");
        let mut command = Command::from_path(&mock_path)?;

        command.args(&["1"]);

        let mut command = command.run()?;

        command.wait().await?;

        Ok(())
    }
}
