use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mock_json::mock;

use crate::{AppError, AppState, UserData};

pub async fn handle(
    state: State<Arc<AppState>>,
    user_id: Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_id.0;
    let mut user: UserData = serde_json::from_str(&mock(&state.user_template).to_string())?;
    user.id = user_id;
    Ok(Json(user))
}
