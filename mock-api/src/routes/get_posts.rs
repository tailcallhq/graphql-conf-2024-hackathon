use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::{AppError, AppState};

pub async fn handle(_state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    // TODO: fill logic
    Ok(())
}
