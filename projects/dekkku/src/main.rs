use async_graphql::dataloader::*;
use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_poem::*;
use bytes::Bytes;
use dashmap::DashMap;
use poem::{listener::TcpListener, web::Html, *};
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::sync::oneshot;
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;

const BASE_URL: &str = "http://localhost:3000";
const ALL_USERS: &str = "http://localhost:3000/users";
const ALL_POSTS: &str = "http://localhost:3000/posts";

struct Store {
    posts: RwLock<HashMap<i32, Post>>,
    users: RwLock<HashMap<i32, User>>,
    is_dirty: RwLock<bool>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            posts: RwLock::new(HashMap::new()),
            users: RwLock::new(HashMap::new()),
            is_dirty: RwLock::new(false),
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
        let http_client = ctx.data_unchecked::<Arc<RequestBatcher>>();
        let posts: Vec<Post> = http_client.request(ALL_POSTS).await?;
        let store = ctx.data_unchecked::<Arc<Store>>();

        let are_posts_same = {
            let is_cache_empty = store.posts.read().unwrap().is_empty();
            if is_cache_empty {
                let mut rng = rand::thread_rng();
                let selected_posts: Vec<&Post> = posts.choose_multiple(&mut rng, 2).collect();
                if selected_posts.len() == 2 {
                    let mut posts_writer = store.posts.write().unwrap();
                    for post in selected_posts {
                        posts_writer.insert(post.id, post.clone());
                    }
                }
                false
            } else {
                let cached_posts: Vec<Post> =
                    store.posts.read().unwrap().values().cloned().collect();
                let mut posts_writer = store.posts.write().unwrap();
                cached_posts.iter().all(|post| {
                    if let Some(new_post) = posts.iter().find(|p| p.id == post.id) {
                        if post != new_post {
                            posts_writer.insert(new_post.id, new_post.clone());
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                })
            }
        };

        if !are_posts_same {
            // clean up the users.
            store.users.write().unwrap().clear();
        }

        *store.is_dirty.write().unwrap() = !are_posts_same;
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
            let store = ctx.data_unchecked::<Arc<Store>>();
            let mut are_posts_same = true;
            if let Some(cached_post) = store.posts.read().unwrap().get(&id) {
                if actual_post != cached_post {
                    *store.is_dirty.write().unwrap() = true;
                    are_posts_same = false;
                }
            }
            if !are_posts_same {
                // clean up the users.
                store.users.write().unwrap().clear();
            }
            store.posts.write().unwrap().insert(id, actual_post.clone());
        }
        Ok(post)
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<User>, async_graphql::Error> {
        let http_client = ctx.data_unchecked::<Arc<RequestBatcher>>();
        let users: Vec<User> = http_client.request(ALL_USERS).await?;

        let store = ctx.data_unchecked::<Arc<Store>>();
        let mut rng = rand::thread_rng();
        let selected_users: Vec<&User> = users.choose_multiple(&mut rng, 2).collect();

        if selected_users.len() == 2 {
            let users_writer = store.users.read().unwrap();
            let are_users_same = selected_users.iter().all(|user| {
                users_writer
                    .get(&user.id)
                    .map_or(false, |cached_user| cached_user == *user)
            });

            if !are_users_same {
                *store.is_dirty.write().unwrap() = true;
            }
        }

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
            let store = ctx.data_unchecked::<Arc<Store>>();
            if let Some(cached_user) = store.users.read().unwrap().get(&id) {
                if cached_user != actual_user {
                    *store.is_dirty.write().unwrap() = true;
                }
            }
            store.users.write().unwrap().insert(id, actual_user.clone());
        }
        Ok(user)
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
struct Post {
    id: i32,
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
        if !*store.is_dirty.read().unwrap()
            && store.users.read().unwrap().contains_key(&self.user_id)
        {
            let user = store.users.read().unwrap().get(&self.user_id).cloned();
            Ok(user)
        } else {
            let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
            let user = loader.load_one(self.user_id).await?;

            if let Some(actual_user) = user.as_ref() {
                if let Some(cached_user) = store.users.read().unwrap().get(&self.user_id) {
                    if cached_user != actual_user {
                        *store.is_dirty.write().unwrap() = true;
                    }
                }
                store
                    .users
                    .write()
                    .unwrap()
                    .insert(self.user_id, actual_user.clone());
            }
            Ok(user)
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
struct User {
    id: i32,
    name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    address: Option<Address>,
    phone: Option<String>,
    website: Option<String>,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Address {
    zipcode: Option<String>,
    geo: Option<Geo>,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Geo {
    lat: Option<f64>,
    lng: Option<f64>,
}

struct PostLoader(Arc<RequestBatcher>);
struct UserLoader(Arc<RequestBatcher>);

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
            let post: Post = self.0.request(&url).await?;
            result.insert(id, post);
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
        let users: Vec<User> = self.0.request(&url).await?;
        for user in users {
            result.insert(user.id, user);
        }
        Ok(result)
    }
}

async fn fetch_with_retry(
    client: &Client,
    url: &str,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3); // Retry up to 3 times

    Retry::spawn(retry_strategy, || client.get(url).send()).await
}

#[allow(dead_code)]
struct HttpClient {
    client: Arc<Client>,
}

impl HttpClient {
    fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    #[allow(dead_code)]
    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
    ) -> std::result::Result<T, anyhow::Error> {
        let response = fetch_with_retry(&self.client, url).await?;
        Ok(response.json().await?)
    }
}

struct RequestBatcher {
    client: Arc<Client>,
    on_going_req:
        Arc<DashMap<String, Vec<oneshot::Sender<std::result::Result<Bytes, anyhow::Error>>>>>,
}

impl RequestBatcher {
    fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            on_going_req: Default::default(),
        }
    }

    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
    ) -> std::result::Result<T, anyhow::Error> {
        let (tx, rx) = oneshot::channel();

        let should_fetch = {
            let mut entry = self.on_going_req.entry(url.to_owned()).or_default();
            entry.push(tx);
            entry.len() == 1
        };

        if should_fetch {
            let client = self.client.clone();
            let url = url.to_owned();
            let on_going_req = self.on_going_req.clone();

            tokio::spawn(async move {
                let result = fetch_with_retry(&client, &url).await;

                if let Ok(response) = result {
                    if let Ok(body) = response.bytes().await {
                        // let's say in queue, we've got around 10k requests and only one is executing right now.
                        // and if it fails then we won't reach here to remove the all requests from queue
                        // handle that scenario.
                        let senders = { on_going_req.remove(&url) };
                        if let Some((_, senders)) = senders {
                            for sender in senders {
                                let _ = sender.send(Ok(body.clone()));
                            }
                        }
                    }
                }
            });
        }

        let response = rx.await??;

        Ok(serde_json::from_slice(&response)?)
    }
}

fn create_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    let client = Arc::new(Client::new());
    let http_client = Arc::new(HttpClient::new(client.clone()));
    let batch_client = Arc::new(RequestBatcher::new(client.clone()));

    let user_loader = DataLoader::new(UserLoader(batch_client.clone()), tokio::spawn)
        .delay(Duration::from_millis(1));
    let post_loader = DataLoader::new(PostLoader(batch_client.clone()), tokio::spawn)
        .delay(Duration::from_millis(5));

    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(http_client)
        .data(user_loader)
        .data(post_loader)
        .data(batch_client)
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
