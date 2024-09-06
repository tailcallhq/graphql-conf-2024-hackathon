use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::{AppError, AppState};

pub async fn handle(
    state: State<Arc<AppState>>,
    post_id: Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let post_id = post_id.0;
    match state.db.post(post_id) {
        Some(post) => Ok(Json(post)),
        None => Err(AppError::NotFound(format!(
            "Post with id {} not found",
            post_id
        ))),
    }
}
