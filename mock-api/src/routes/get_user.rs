use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{AppError, AppState};

pub async fn handle(
    _state: State<Arc<AppState>>,
    _user_id: Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: fill logic
    Ok(())
}
