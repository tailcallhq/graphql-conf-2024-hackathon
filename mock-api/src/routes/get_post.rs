use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mock_json::mock;

use crate::{AppError, AppState, PostData};

pub async fn handle(
    state: State<Arc<AppState>>,
    post_id: Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let post_id = post_id.0;
    let mut post: PostData = serde_json::from_str(&mock(&state.post_template).to_string())?;
    post.id = post_id;
    Ok(Json(post))
}
