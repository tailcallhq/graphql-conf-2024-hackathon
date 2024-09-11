use async_graphql::dataloader::*;
use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

const BASE_URL: &str = "http://localhost:3000";
const ALL_USERS: &str = "http://localhost:3000/users";
const ALL_POSTS: &str = "http://localhost:3000/posts";

struct Store {
    post: RwLock<HashMap<i32, Post>>,
    users: RwLock<HashMap<i32, User>>,
    is_db_same: RwLock<bool>, // rename to is_dirty.
}

impl Default for Store {
    fn default() -> Self {
        Self {
            post: RwLock::new(HashMap::default()),
            users: RwLock::new(HashMap::default()),
            is_db_same: RwLock::new(false),
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

        let response = client.get(ALL_POSTS).send().await?;
        let posts: Vec<Post> = response.json().await?;

        let store = ctx.data_unchecked::<Arc<Store>>();

        let are_posts_same = {
            // TODO: pick ID's
            let is_cache_empty = { store.post.read().unwrap().is_empty() };

            if is_cache_empty {
                // Pick any two random posts from posts
                let mut rng = rand::thread_rng();
                let selected_posts: Vec<&Post> = posts.choose_multiple(&mut rng, 2).collect();
                if selected_posts.len() == 2 {
                    let post1 = selected_posts[0];
                    let post2 = selected_posts[1];
                    let mut posts_writer = store.post.write().unwrap();
                    posts_writer.insert(post1.id.unwrap(), post1.clone());
                    posts_writer.insert(post2.id.unwrap(), post2.clone());
                }
                false
            } else {
                // We've the posts, now check the posts from cache with the posts.
                let mut are_posts_same = true;
                let cached_posts = {
                    store
                        .post
                        .write()
                        .unwrap()
                        .values()
                        .cloned()
                        .collect::<Vec<_>>()
                };
                let mut posts_writer = store.post.write().unwrap();
                for post in cached_posts {
                    if let Some(new_post) = posts.iter().find(|p| p.id == post.id) {
                        if post != *new_post {
                            are_posts_same = false;
                            posts_writer.insert(new_post.id.unwrap(), new_post.clone());
                        }
                    } else {
                        are_posts_same = false;
                        break;
                    }
                }
                are_posts_same
            }
        };

        *store.is_db_same.write().unwrap() = are_posts_same;

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
            if let Some(cached_post) = store.post.write().unwrap().get(&id) {
                if actual_post != cached_post {
                    *store.is_db_same.write().unwrap() = false;
                }
            }
            store.post.write().unwrap().insert(id, actual_post.clone());
        }
        Ok(post)
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<User>, async_graphql::Error> {
        let client = ctx.data_unchecked::<Arc<Client>>();
        let response = client.get(ALL_USERS).send().await?;
        let users: Vec<User> = response.json().await?;

        let store = ctx.data_unchecked::<Arc<Store>>();
        let mut rng = rand::thread_rng();
        let selected_users: Vec<&User> = users.choose_multiple(&mut rng, 2).collect();

        if selected_users.len() == 2 {
            let user1 = selected_users[0];
            let user2 = selected_users[1];

            let users_writer = store.users.read().unwrap();
            let mut are_users_same = true;

            for user in [user1, user2] {
                if let Some(id) = user.id {
                    if let Some(cached_user) = users_writer.get(&id) {
                        if cached_user != user {
                            are_users_same = false;
                        }
                    } else {
                        are_users_same = false;
                    }
                }
            }

            if !are_users_same {
                *store.is_db_same.write().unwrap() = false;
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
            // cache the user early
            let store = ctx.data_unchecked::<Arc<Store>>();
            if let Some(cached_user) = store.users.write().unwrap().get(&id) {
                if cached_user != actual_user {
                    *store.is_db_same.write().unwrap() = false;
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
        if *store.is_db_same.read().unwrap()
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
                        *store.is_db_same.write().unwrap() = false;
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
    id: Option<i32>,
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
        DataLoader::new(PostLoader(client.clone()), tokio::spawn).delay(Duration::from_millis(5));
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
