use async_graphql::dataloader::*;
use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock}, // Change Mutex to RwLock
    time::Duration,
};

const BASE_URL: &str = "http://localhost:3000";

struct Store {
    post: RwLock<HashMap<i32, Post>>,  // Change Mutex to RwLock
    users: RwLock<HashMap<i32, User>>, // Change Mutex to RwLock
    is_post_same: RwLock<bool>,        // Change Mutex to RwLock
}

impl Default for Store {
    fn default() -> Self {
        Self {
            post: RwLock::new(HashMap::default()), // Change Mutex to RwLock
            users: RwLock::new(HashMap::default()), // Change Mutex to RwLock
            is_post_same: RwLock::new(false),      // Change Mutex to RwLock
        }
    }
}

impl Store {
    pub fn reset(&self) {
        self.users.write().unwrap().clear(); // Change lock to write()
        self.post.write().unwrap().clear(); // Change lock to write()
        *self.is_post_same.write().unwrap() = false; // Change lock to write()
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

        let store = ctx.data_unchecked::<Arc<Store>>();

        let is_same = {
            // TODO: pick these ID's randomly to reduce the chance of false positives.
            let are_posts_same = {
                let cached_post = store.post.read().unwrap();
                let post1_exists = cached_post
                    .get(&posts[1].id.unwrap())
                    .map(|post1| *post1 == posts[1])
                    .unwrap_or(false);
                let post2_exists = cached_post
                    .get(&posts[2].id.unwrap())
                    .map(|post2| post2 == &posts[2])
                    .unwrap_or(false);

                post1_exists && post2_exists
            };

            if are_posts_same {
                true
            } else {
                store.reset();
                store
                    .post
                    .write()
                    .unwrap()
                    .insert(posts[1].id.unwrap(), posts[1].clone());
                store
                    .post
                    .write()
                    .unwrap()
                    .insert(posts[2].id.unwrap(), posts[2].clone());
                false
            }
        };

        {
            let mut is_post_same = store.is_post_same.write().unwrap();
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
        if let Some(actual_post) = post.as_ref() {
            // cache the post early
            let store = ctx.data_unchecked::<Arc<Store>>();
            store.post.write().unwrap().insert(id, actual_post.clone());
        }
        Ok(post)
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<User>, async_graphql::Error> {
        // TODO: cache all the users here.
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
        let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
        let user = loader.load_one(id).await?;

        if let Some(actual_user) = &user {
            // cache the user early
            let store = ctx.data_unchecked::<Arc<Store>>();
            store.users.write().unwrap().insert(id, actual_user.clone());
        }

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
        if *store.is_post_same.read().unwrap()
            && store.users.read().unwrap().contains_key(&self.user_id)
        {
            let user = store.users.write().unwrap().get(&self.user_id).cloned();
            Ok(user)
        } else {
            let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
            let user = loader.load_one(self.user_id).await?;
            if let Some(actual_user) = user.as_ref() {
                let mut user_store = store.users.write().unwrap();
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

// 1. Connection Pooling.
// 2. reset property of Mock Server.
