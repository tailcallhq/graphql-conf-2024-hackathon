use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{AppError, AppState};

pub async fn handle(state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    Ok(Json(state.db.posts()))
}
