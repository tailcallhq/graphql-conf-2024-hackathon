use std::fs;

use anyhow::Result;
use clap::Parser;
use tracing::{error, info};

use hackathon::project::Project;
use hackathon::ROOT_DIR;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    project: Option<String>,
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let projects_dir = format!("{ROOT_DIR}/projects");

    for entry in fs::read_dir(projects_dir)? {
        let path = entry?.path();

        if path.is_dir() {
            let project = Project::new(path)?;

            if let Some(only_project) = &args.project {
                if project.name() != only_project {
                    info!("Ignore project: {}", project.name());
                    continue;
                }
            }

            project.run_project().await?;
        }
    }

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
