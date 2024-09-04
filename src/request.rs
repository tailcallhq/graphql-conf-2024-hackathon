use anyhow::Result;
use serde_json::json;

const API_URL: &str = "http://localhost:8000/graphql";

pub async fn graphql_request(query: &str) -> Result<serde_json::Value> {
    let client = reqwest::Client::new();

	let value = json!({
		"operationName": null,
		"variables": {},
		"query": query
	});

    let response = client.post(API_URL).json(&value).send().await?;

    Ok(response.json().await?)
}
