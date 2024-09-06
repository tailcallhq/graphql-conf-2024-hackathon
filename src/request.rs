use anyhow::Result;
use serde_json::json;

pub struct GraphqlClient {
    api: &'static str,
}

pub const TESTED_GRAPHQL_CLIENT: GraphqlClient = GraphqlClient {
    api: "http://localhost:8000/graphql",
};

pub const REFERENCE_GRAPHQL_CLIENT: GraphqlClient = GraphqlClient {
    api: "http://localhost:8089/graphql",
};

impl GraphqlClient {
    pub async fn request(&self, query: &str) -> Result<serde_json::Value> {
        let client = reqwest::Client::new();

        let value = json!({
            "operationName": null,
            "variables": {},
            "query": query
        });

        let response = client.post(self.api).json(&value).send().await?;

        Ok(response.json().await?)
    }
}
