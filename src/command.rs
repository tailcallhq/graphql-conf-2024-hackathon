use std::path::Path;

use anyhow::{anyhow, Result};
use command_group::{AsyncCommandGroup, AsyncGroupChild};
use tracing::info;

pub struct Command {
    command: tokio::process::Command,
}

impl Command {
    pub fn from_path(cmd_path: &Path) -> Result<Self> {
        let name = cmd_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("cmd");

        if !cmd_path.exists() {
            return Err(anyhow!(
                "{name} file not found at path: `{}`.
    This file is required to run the server.
            ",
                cmd_path.display()
            ));
        }

        info!("Running file `{}`", cmd_path.display());

        let mut command = tokio::process::Command::new(cmd_path);

        command.current_dir(&cmd_path.parent().unwrap_or(cmd_path));

        Ok(Self { command })
    }

    pub fn args(&mut self, args: &[&str]) {
        self.command.args(args);
    }

    pub fn run(&mut self) -> Result<AsyncGroupChild> {
        let mut command = self.command.group();

        command.kill_on_drop(true);

        info!("Output logs from setup script below");

        let child = command.spawn()?;

        Ok(child)
    }
}
