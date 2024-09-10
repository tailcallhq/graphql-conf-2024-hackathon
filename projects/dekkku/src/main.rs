use async_graphql::dataloader::*;
use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

const BASE_URL: &str = "http://localhost:3000";

struct Store {
    post: Mutex<Post>,
    users: Mutex<HashMap<i32, User>>,
    is_post_same: Mutex<bool>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            post: Mutex::new(Post::default()),
            users: Mutex::new(HashMap::default()),
            is_post_same: Mutex::new(false),
        }
    }
}

struct Query;

#[Object]
impl Query {
    async fn posts(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<Post>, async_graphql::Error> {
        let client = ctx.data_unchecked::<Arc<Client>>();

        let response = client.get(format!("{}/posts", BASE_URL)).send().await?;
        let posts: Vec<Post> = serde_json::from_value(response.json().await?)?;

        // exploit the `reset` property of Mock Server.

        let store = ctx.data_unchecked::<Arc<Store>>();

        let is_same = {
            let mut cached_post = store.post.lock().unwrap();
            let is_same = (*cached_post) == posts[0];
            if is_same {
                true
            } else {
                *cached_post = posts[0].clone();
                false
            }
        };

        {
            let mut is_post_same = store.is_post_same.lock().unwrap();
            *is_post_same = is_same;
        }

        Ok(posts)
    }

    async fn post(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> std::result::Result<Option<Post>, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<PostLoader>>();
        let post = loader.load_one(id).await?;
        Ok(post)
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<User>, async_graphql::Error> {
        let client = ctx.data_unchecked::<Arc<Client>>();
        let response = client.get(format!("{}/users", BASE_URL)).send().await?;
        let users: Vec<User> = serde_json::from_value(response.json().await?)?;
        Ok(users)
    }

    async fn user(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> std::result::Result<Option<User>, async_graphql::Error> {
        // TODO: cache the user for future purpose.
        let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
        let user = loader.load_one(id).await?;
        Ok(user)
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
struct Post {
    id: Option<i32>,
    #[graphql(name = "userId")]
    user_id: i32,
    title: Option<String>,
    body: Option<String>,
}

#[ComplexObject]
impl Post {
    async fn user(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Option<User>, async_graphql::Error> {
        let store = ctx.data_unchecked::<Arc<Store>>();
        if *store.is_post_same.lock().unwrap() {
            let user = store.users.lock().unwrap().get(&self.user_id).cloned();
            Ok(user)
        } else {
            let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
            let user = loader.load_one(self.user_id).await?;
            if let Some(actual_user) = user.as_ref() {
                let mut user_store = store.users.lock().unwrap();
                user_store.insert(self.user_id, actual_user.clone());
            }
            Ok(user)
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
struct User {
    id: Option<i32>,
    name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    address: Option<Address>,
    phone: Option<String>,
    website: Option<String>,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
struct Address {
    zipcode: Option<String>,
    geo: Option<Geo>,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
struct Geo {
    lat: Option<f64>,
    lng: Option<f64>,
}

struct PostLoader(Arc<Client>);
struct UserLoader(Arc<Client>);

impl Loader<i32> for PostLoader {
    type Value = Post;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        keys: &[i32],
    ) -> std::result::Result<std::collections::HashMap<i32, Self::Value>, Self::Error> {
        let mut result = std::collections::HashMap::new();
        for &id in keys {
            let url = format!("{}/posts/{}", BASE_URL, id);
            let response = self.0.get(url).send().await?;
            if response.status().is_success() {
                let post: Post = response.json().await?;
                result.insert(id, post);
            }
        }
        Ok(result)
    }
}

impl Loader<i32> for UserLoader {
    type Value = User;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        keys: &[i32],
    ) -> std::result::Result<std::collections::HashMap<i32, Self::Value>, Self::Error> {
        let mut result = std::collections::HashMap::new();
        let qp = keys
            .iter()
            .map(|id| format!("id={}", id))
            .collect::<Vec<_>>()
            .join("&");
        let url = format!("{}/users?{}", BASE_URL, qp);
        let response = self.0.get(url).send().await?;
        if response.status().is_success() {
            let users: Vec<User> = response.json().await?;
            for user in users {
                if let Some(id) = user.id {
                    result.insert(id, user);
                }
            }
        }
        Ok(result)
    }
}

fn create_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    let client = Arc::new(Client::new());
    let user_loader =
        DataLoader::new(UserLoader(client.clone()), tokio::spawn).delay(Duration::from_millis(1));
    let post_loader =
        DataLoader::new(PostLoader(client.clone()), tokio::spawn).delay(Duration::from_millis(1));
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(client)
        .data(user_loader)
        .data(post_loader)
        .data(Arc::new(Store::default()))
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
    let app = Route::new()
        .at("/graphql", get(graphiql).post(GraphQL::new(schema.clone())))
        .at("/", get(graphiql).post(GraphQL::new(schema.clone())));
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}

// Ideas
// 1. Connection Pooling.
// 2. Caching based on POST's.
