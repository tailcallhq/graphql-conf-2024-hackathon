use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::{AppError, AppState};

/// route handler for resetting the database
pub async fn handle(state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    match state.db.reset() {
        Ok(()) => Ok(Json(json!({"status": "Database reset successfully"}))),
        Err(_e) => Err(AppError::InternalServerError(
            "Failed to reset database".to_string(),
        )),
    }
}
