use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::{AppError, AppState};

pub async fn handle(state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let _ = state.db.update()?;
    Ok(Json(json!({})))
}
