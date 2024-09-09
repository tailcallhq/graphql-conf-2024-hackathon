use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

use crate::{AppError, AppState};

pub async fn handle(
    state: State<Arc<AppState>>,
    Query(params): Query<Vec<(String, i64)>>,
) -> Result<impl IntoResponse, AppError> {
    let users = if params.is_empty() {
        state.db.users()
    } else {
        params.into_iter()
            .filter(|(key, _)| key == "id")
            .filter_map(|(_, id)| state.db.user(id))
            .collect()
    };

    Ok(Json(users))
}
