use std::path::Path;

use anyhow::Result;
use hackathon::{project::Project, ROOT_DIR};
use tracing::error;

async fn run() -> Result<()> {
    let project = Project::new(Path::new(ROOT_DIR).join("projects/tailcallhq"))?;

    project.run_baseline().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(error) = run().await {
        error!("Critical error: {:#}", error);
        panic!("Critical error");
    }
}
