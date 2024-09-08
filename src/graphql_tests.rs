use std::{fs, sync::LazyLock};

use anyhow::{anyhow, Result};
use diff_logger::DiffLogger;
use reqwest::Method;
use tracing::{error, info};

use crate::request::{MOCK_API_CLIENT, REFERENCE_GRAPHQL_CLIENT, TESTED_GRAPHQL_CLIENT};

use super::ROOT_DIR;

const NUMBER_OF_TESTS: usize = 5;

static TESTS: LazyLock<Result<Vec<String>>> = LazyLock::new(|| {
    let tests_path = format!("{ROOT_DIR}/tests");
    let mut tests = Vec::new();

    for entry in fs::read_dir(tests_path)? {
        let path = entry?.path();

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext == "graphql" {
                tests.push(fs::read_to_string(&path)?);
            }
        }
    }

    Ok(tests)
});

pub async fn run_graphql_tests() -> Result<()> {
    info!("Run graphql assert tests");

    let tests = TESTS
        .as_ref()
        .map_err(|e| anyhow!("Failed to resolve tests due to error: {e}"))?;

    for i in 1..NUMBER_OF_TESTS {
        info!("Test iteration: {i}");

        MOCK_API_CLIENT.request(Method::POST, "reset").await?;

        for test in tests {
            let actual = TESTED_GRAPHQL_CLIENT.request(&test).await?;

            let expected = REFERENCE_GRAPHQL_CLIENT.request(&test).await?;

            let differ = DiffLogger::new();

            let difference = differ.diff(&expected, &actual);

            if !difference.is_empty() {
                error!(
                    "Actual response is not equal to expected
    Note: left is expected response -> right is actual response"
                );
                println!("{}", difference);

                return Err(anyhow!("Actual response is not equal to expected"));
            }
        }
    }

    info!("Execution of graphql tests finished");

    Ok(())
}
