use std::fs;

use anyhow::Result;
use tracing::error;

use project::Project;

mod command;
mod project;
mod request;
mod graphql_tests;
mod utils;

pub const ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");

async fn run() -> Result<()> {
    let projects_dir = format!("{ROOT_DIR}/projects");

    for entry in fs::read_dir(projects_dir)? {
        let path = entry?.path();

        if path.is_dir() {
            let project = Project::new(path)?;

            project.run_project().await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(error) = run().await {
        error!("Critical error: {}", error);
        panic!("Critical error");
    }
}
