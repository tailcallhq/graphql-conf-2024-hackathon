use std::{sync::Arc, time::Duration};

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use mock_api::{utils::env_default, AppState};
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

    let delay = env_default("MOCK_SERVER_DELAY", 5);
    let delay = Duration::from_millis(delay);

    let burst_size = env_default("MOCK_SERVER_BURST_SIZE", 1000);

    let rate_limiter_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_nanosecond((1000000000 / burst_size).into())
            .burst_size(burst_size)
            .key_extractor(GlobalKeyExtractor)
            .finish()
            .unwrap(),
    );

    let state = Arc::new(AppState::default());

    let mut router = Router::new()
        .route(
            "/",
            get(|| async { (StatusCode::OK, "BENCHING").into_response() }),
        )
        .route("/posts", get(mock_api::routes::get_posts::handle))
        .route("/posts/:post_id", get(mock_api::routes::get_post::handle))
        .route("/users", get(mock_api::routes::get_users::handle))
        .route("/users/:user_id", get(mock_api::routes::get_user::handle))
        .route("/reset", post(mock_api::routes::reset_database::handle))
        .layer(axum::middleware::from_fn(
            move |request: Request, next: Next| {
                let delay = delay.clone();
                async move {
                    let response = next.run(request).await;
                    tokio::time::sleep(delay).await;
                    response
                }
            },
        ))
        .with_state(state);

    if env_default("MOCK_SERVER_LIMITER_ENABLED", false) {
        router = router.layer(GovernorLayer {
            config: rate_limiter_config,
        })
    }

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
