use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use futures::future::join_all;
use moka::future::Cache;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

const BASE_URL: &str = "http://localhost:3000";
const CACHE_MAX_CAPACITY: u64 = 10_000;
const CACHE_TIME_TO_LIVE: Duration = Duration::from_secs(60 * 5); // 5 minutes

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Post {
    id: i32,
    user_id: i32,
    title: Option<String>,
    body: Option<String>,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
struct User {
    id: i32,
    name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    address: Option<Address>,
    phone: Option<String>,
    website: Option<String>,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
struct Address {
    zipcode: Option<String>,
    geo: Option<Geo>,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
struct Geo {
    lat: Option<f64>,
    lng: Option<f64>,
}

struct Query;

#[Object]
impl Query {
    async fn posts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Post>> {
        let loader = ctx.data::<Arc<Loader>>().unwrap();
        let posts = loader.load_posts(vec![]).await?;
        Ok(posts)
    }

    async fn post(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Option<Post>> {
        let loader = ctx.data::<Arc<Loader>>().unwrap();
        let posts = loader.load_posts(vec![id]).await?;
        Ok(posts.into_iter().next())
    }

    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let loader = ctx.data::<Arc<Loader>>().unwrap();
        let users = loader.load_users(vec![]).await?;
        Ok(users)
    }

    async fn user(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Option<User>> {
        let loader = ctx.data::<Arc<Loader>>().unwrap();
        let users = loader.load_users(vec![id]).await?;
        Ok(users.into_iter().next())
    }
}

#[Object]
impl Post {
    async fn id(&self) -> i32 {
        self.id
    }

    #[graphql(name = "userId")]
    async fn user_id(&self) -> i32 {
        self.user_id
    }

    async fn title(&self) -> &Option<String> {
        &self.title
    }

    async fn body(&self) -> &Option<String> {
        &self.body
    }

    async fn user(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<User>> {
        let loader = ctx.data::<Arc<Loader>>().unwrap();
        let users = loader.load_users(vec![self.user_id]).await?;
        Ok(users.into_iter().next())
    }
}

struct Loader {
    client: Client,
    post_cache: Cache<String, Vec<Post>>,
    user_cache: Cache<String, Vec<User>>,
}

impl Loader {
    async fn load_posts(&self, ids: Vec<i32>) -> Result<Vec<Post>, reqwest::Error> {
        if ids.is_empty() {
            if let Some(posts) = self.post_cache.get("all_posts") {
                return Ok(posts);
            }

            let posts: Vec<Post> = self.client
                .get(&format!("{}/posts", BASE_URL))
                .send()
                .await?
                .json()
                .await?;

            self.post_cache.insert("all_posts".to_string(), posts.clone()).await;
            return Ok(posts);
        }

        let fetches = ids.into_iter().map(|id| {
            let cache_key = format!("post_{}", id);
            let client = self.client.clone();
            let cache = self.post_cache.clone();

            async move {
                if let Some(posts) = cache.get(&cache_key) {
                    return Ok(posts[0].clone());
                }

                let post: Post = client
                    .get(&format!("{}/posts/{}", BASE_URL, id))
                    .send()
                    .await?
                    .json()
                    .await?;

                cache.insert(cache_key, vec![post.clone()]).await;
                Ok(post)
            }
        });

        join_all(fetches).await.into_iter().collect()
    }

    async fn load_users(&self, ids: Vec<i32>) -> Result<Vec<User>, reqwest::Error> {
        if ids.is_empty() {
            if let Some(users) = self.user_cache.get("all_users") {
                return Ok(users);
            }

            let users: Vec<User> = self.client
                .get(&format!("{}/users", BASE_URL))
                .send()
                .await?
                .json()
                .await?;

            self.user_cache.insert("all_users".to_string(), users.clone()).await;
            return Ok(users);
        }

        let fetches = ids.into_iter().map(|id| {
            let cache_key = format!("user_{}", id);
            let client = self.client.clone();
            let cache = self.user_cache.clone();

            async move {
                if let Some(users) = cache.get(&cache_key) {
                    return Ok(users[0].clone());
                }

                let user: User = client
                    .get(&format!("{}/users/{}", BASE_URL, id))
                    .send()
                    .await?
                    .json()
                    .await?;

                cache.insert(cache_key, vec![user.clone()]).await;
                Ok(user)
            }
        });

        join_all(fetches).await.into_iter().collect()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client");

    let post_cache: Cache<String, Vec<Post>> = Cache::builder()
        .max_capacity(CACHE_MAX_CAPACITY)
        .time_to_live(CACHE_TIME_TO_LIVE)
        .build();

    let user_cache: Cache<String, Vec<User>> = Cache::builder()
        .max_capacity(CACHE_MAX_CAPACITY)
        .time_to_live(CACHE_TIME_TO_LIVE)
        .build();

    let loader = Arc::new(Loader {
        client,
        post_cache,
        user_cache,
    });

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(loader)
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").to(graphiql))
            .service(web::resource("/graphql").to(graphql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn graphiql() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql(schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}