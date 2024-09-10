use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const BASE_URL: &str = "http://localhost:3000";

struct Query;

#[Object]
impl Query {
    async fn posts(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Post>> {
        let client = ctx.data_unchecked::<Arc<Client>>();
        let response = client.get(format!("{}/posts", BASE_URL)).send().await?;
        let posts: serde_json::Value = response.json().await?;
        let posts: Vec<Post> = serde_json::from_value(posts)?;
        Ok(posts)
    }

    async fn post(&self, ctx: &Context<'_>, id: i32) -> anyhow::Result<Option<Post>> {
        let client = ctx.data_unchecked::<Arc<Client>>();
        let response = client
            .get(format!("{}/posts/{}", BASE_URL, id))
            .send()
            .await?;
        if response.status().is_success() {
            let post: serde_json::Value = response.json().await?;
            let post = serde_json::from_value(post)?;
            Ok(Some(post))
        } else {
            Ok(None)
        }
    }

    async fn users(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<User>> {
        let client = ctx.data_unchecked::<Arc<Client>>();
        let response = client.get(format!("{}/users", BASE_URL)).send().await?;
        let users: serde_json::Value = response.json().await?;
        let users: Vec<User> = serde_json::from_value(users)?;
        Ok(users)
    }

    async fn user(&self, ctx: &Context<'_>, id: i32) -> anyhow::Result<Option<User>> {
        let client = ctx.data_unchecked::<Arc<Client>>();
        let response = client
            .get(format!("{}/users/{}", BASE_URL, id))
            .send()
            .await?;
        if response.status().is_success() {
            let user: serde_json::Value = response.json().await?;
            let user = serde_json::from_value(user)?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
struct Post {
    id: Option<i32>,
    #[graphql(name = "userId")]
    user_id: i32,
    title: Option<String>,
    body: Option<String>,
}

#[ComplexObject]
impl Post {
    async fn user(&self, ctx: &Context<'_>) -> anyhow::Result<User> {
        let client = ctx.data_unchecked::<Arc<Client>>();
        let response = client
            .get(format!("{}/users/{}", BASE_URL, self.user_id))
            .send()
            .await?;
        let user: serde_json::Value = response.json().await?;
        let user = serde_json::from_value(user)?;
        Ok(user)
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
struct User {
    id: Option<i32>,
    name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    address: Option<Address>,
    phone: Option<String>,
    website: Option<String>,
}

#[ComplexObject]
impl User {
    async fn posts(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Post>> {
        let client = ctx.data_unchecked::<Arc<Client>>();
        let response = client
            .get(format!("{}/posts?userId={}", BASE_URL, self.id.unwrap()))
            .send()
            .await?;
        let posts: Vec<Post> = response.json().await?;
        Ok(posts)
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
struct Address {
    zipcode: Option<String>,
    geo: Option<Geo>,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
struct Geo {
    lat: Option<f64>,
    lng: Option<f64>,
}

fn create_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(Arc::new(Client::new()))
        .finish()
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = create_schema();

    // start the http server
    let app = Route::new().at("/graphql", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
