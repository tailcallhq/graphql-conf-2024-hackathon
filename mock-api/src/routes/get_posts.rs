use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{AppError, AppState};

/// route handler for getting all posts
pub async fn handle(state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    Ok(Json(state.db.posts()))
}
