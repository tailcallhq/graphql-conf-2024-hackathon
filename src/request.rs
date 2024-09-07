use anyhow::Result;
use reqwest::Method;
use serde_json::json;

pub struct GraphqlClient {
    api: &'static str,
}

pub struct RestClient {
    api: &'static str,
}

pub const TESTED_GRAPHQL_CLIENT: GraphqlClient = GraphqlClient {
    api: "http://localhost:8000/graphql",
};

pub const REFERENCE_GRAPHQL_CLIENT: GraphqlClient = GraphqlClient {
    api: "http://localhost:8089/graphql",
};

pub const MOCK_API_CLIENT: RestClient = RestClient {
    api: "http://localhost:3000",
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

impl RestClient {
    pub async fn request(&self, method: Method, path: &str) -> Result<serde_json::Value> {
        let client = reqwest::Client::new();

        let response = client
            .request(method, format!("{}/{}", self.api, path))
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
