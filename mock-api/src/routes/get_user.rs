use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::{AppError, AppState};

pub async fn handle(
    state: State<Arc<AppState>>,
    user_id: Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_id.0;
    match state.db.user(user_id) {
        Some(user) => Ok(Json(user).into_response()),
        None => Err(AppError::NotFound(format!("User with id {} not found", user_id))),
    }
}
