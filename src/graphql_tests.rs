use std::{fs, sync::LazyLock};

use anyhow::{anyhow, Result};
use diff_logger::DiffLogger;
use reqwest::Method;
use tracing::{error, info};

use crate::{
    query_info::Schema,
    request::{MOCK_API_CLIENT, REFERENCE_GRAPHQL_CLIENT, TESTED_GRAPHQL_CLIENT},
    type_info::Root,
};

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

fn query_builder(type_name: &str) -> String {
    format!(
        r#"
        {{
          __type(name: "{}") {{
            name
            kind
            fields {{
              name
              args {{
                name
              }}
            }}
          }}
        }}
        "#,
        type_name
    )
}

pub async fn run_introspection_query() -> Result<()> {
    info!("Run graphql introspection tests");
    let query_info = include_str!("./query_info.graphql");

    // check the root query is same or not.
    let actual_value = TESTED_GRAPHQL_CLIENT.request(&query_info).await?;
    let expected_value = REFERENCE_GRAPHQL_CLIENT.request(&query_info).await?;

    let actual: Schema = serde_json::from_value(actual_value.clone())?;
    let expected: Schema = serde_json::from_value(expected_value.clone())?;
    let differ = DiffLogger::new();
    if actual != expected {
        let difference = differ.diff(
            &serde_json::to_value(actual.clone())?,
            &serde_json::to_value(expected.clone())?,
        );
        error!("Query Operation type mismatch");
        println!("{}", difference);
        return Err(anyhow!("Query Operation type mismatch"));
    }

    for (actual, expected) in actual
        .data
        .schema
        .query_type
        .fields
        .iter()
        .zip(expected.data.schema.query_type.fields.iter())
    {
        let actual_op_type = match actual.field_type.get_name() {
            Some(name) => name,
            None => continue,
        };

        let expected_op_type = match expected.field_type.get_name() {
            Some(name) => name,
            None => continue,
        };

        let actual_op_type_query = query_builder(&actual_op_type);
        let expected_op_type_query = query_builder(&expected_op_type);

        let actual: Root =
            serde_json::from_value(TESTED_GRAPHQL_CLIENT.request(&actual_op_type_query).await?)?;
        let expected: Root = serde_json::from_value(
            REFERENCE_GRAPHQL_CLIENT
                .request(&expected_op_type_query)
                .await?,
        )?;

        if actual != expected {
            let difference = differ.diff(
                &serde_json::to_value(actual)?,
                &serde_json::to_value(expected)?,
            );
            error!("Type Defination mismatch");
            println!("{}", difference);
            return Err(anyhow!("Type Defination mismatch"));
        }
    }

    info!("Execution of graphql schema validation finished");

    Ok(())
}
