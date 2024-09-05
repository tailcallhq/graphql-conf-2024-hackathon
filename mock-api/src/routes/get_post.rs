use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{AppError, AppState};

pub async fn handle(
    _state: State<Arc<AppState>>,
    _post_id: Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: fill logic
    Ok(())
}
