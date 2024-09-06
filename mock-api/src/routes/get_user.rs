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
    Ok(Json(state.db.user(user_id)))
}
