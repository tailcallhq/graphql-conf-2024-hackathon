use std::{env, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use mock_api::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    // setup debugging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let _delay: i64 = env::var("BENCH_DELAY")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap();
    let _rate_limit: i64 = env::var("BENCH_LIMIT")
        .unwrap_or("1000".to_string())
        .parse()
        .unwrap();

    // TODO: load mock data from json files
    let state = Arc::new(AppState::default());

    let router = Router::new()
        .route(
            "/",
            get(|| async { (StatusCode::OK, "BENCHING").into_response() }),
        )
        .route("/posts", get(mock_api::routes::get_posts::handle))
        .route("/posts/:post_id", get(mock_api::routes::get_post::handle))
        .route("/users", get(mock_api::routes::get_users::handle))
        .route("/users/:user_id", get(mock_api::routes::get_user::handle))
        .with_state(state);

    // TODO: add rate limiter
    // TODO: add latency layer

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
