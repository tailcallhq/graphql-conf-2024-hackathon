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
    Ok(Json(state.db.post(post_id)))
}
