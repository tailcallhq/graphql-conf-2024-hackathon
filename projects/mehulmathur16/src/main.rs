use actix_web::{guard, web, App, HttpServer};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Post {
    id: i32,
    #[serde(rename = "userId")]
    user_id: i32,
    title: String,
    body: String,
}

#[derive(SimpleObject, Deserialize)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
}

#[derive(SimpleObject, Deserialize)]
struct Address {
    zipcode: String,
    geo: Geo,
}

#[derive(SimpleObject, Deserialize)]
struct Geo {
    lat: f64,
    lng: f64,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn posts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Post>> {
        let client = ctx.data::<Client>().unwrap();
        let response = client
            .get("http://localhost:3000/posts")
            .send()
            .await?
            .json::<Vec<Post>>()
            .await?;
        Ok(response)
    }

    async fn post(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Post> {
        let client = ctx.data::<Client>().unwrap();
        let response = client
            .get(&format!("http://localhost:3000/posts/{}", id))
            .send()
            .await?
            .json::<Post>()
            .await?;
        Ok(response)
    }

    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let client = ctx.data::<Client>().unwrap();
        let response = client
            .get("http://localhost:3000/users")
            .send()
            .await?
            .json::<Vec<User>>()
            .await?;
        Ok(response)
    }

    async fn user(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<User> {
        let client = ctx.data::<Client>().unwrap();
        let response = client
            .get(&format!("http://localhost:3000/users/{}", id))
            .send()
            .await?
            .json::<User>()
            .await?;
        Ok(response)
    }
}

#[Object]
impl Post {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn user_id(&self) -> i32 {
        self.user_id
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn body(&self) -> &str {
        &self.body
    }

    async fn user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let client = ctx.data::<Client>().unwrap();
        let response = client
            .get(&format!("http://localhost:3000/users/{}", self.user_id))
            .send()
            .await?
            .json::<User>()
            .await?;
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Client::new())
        .finish();

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(schema.clone())).service(
            web::resource("/graphql")
                .guard(guard::Post())
                .to(graphql_handler),
        )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

async fn graphql_handler(
    schema: web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
