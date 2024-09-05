use std::{env, sync::Arc, time::Duration};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use mock_api::{delay_middleware::DelayLayer, AppState};
use tokio::net::TcpListener;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::GlobalKeyExtractor, GovernorLayer,
};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    // setup debugging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let delay: u64 = env::var("BENCH_DELAY")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap();

    let burst_size: u32 = env::var("BENCH_BURST_SIZE")
        .unwrap_or("1000".to_string())
        .parse()
        .unwrap();

    let rate_limiter_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_nanosecond((1000000000 / burst_size).into())
            .burst_size(burst_size)
            .key_extractor(GlobalKeyExtractor)
            .finish()
            .unwrap(),
    );

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
        .layer(GovernorLayer {
            config: rate_limiter_config,
        })
        .layer(DelayLayer::new(Duration::from_millis(delay)))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
