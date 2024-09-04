use std::fs;

use anyhow::Result;
use insta::assert_json_snapshot;
use tokio::task::JoinSet;
use tracing::info;

use crate::request::graphql_request;

use super::ROOT_DIR;

pub async fn run_graphql_tests() -> Result<()> {
    info!("Run graphql assert tests");

    let tests_path = format!("{ROOT_DIR}/tests");

    let mut set: JoinSet<Result<()>> = JoinSet::new();

    for entry in fs::read_dir(tests_path)? {
        let path = entry?.path();

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext == "graphql" {
                set.spawn(async {
                    tokio::spawn(async move {
                        let content = tokio::fs::read_to_string(&path).await?;

                        let response = graphql_request(&content).await?;
                        let name =
                            format!("request_{}", path.file_name().unwrap().to_string_lossy());

                        assert_json_snapshot!(name, response);

                        anyhow::Ok(())
                    })
                    .await?
                });
            }
        }
    }

    let results = set.join_all().await;

    info!("Execution of graphql tests finished");

    results.into_iter().collect()
}
