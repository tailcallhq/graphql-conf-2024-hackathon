use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mock_json::mock;

use crate::{AppError, AppState, UserData};

pub async fn handle(state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let users: Vec<UserData> = (1..=10)
        .map(|id| {
            let mut user: UserData = serde_json::from_str(&mock(&state.user_template).to_string())?;
            user.id = id;
            Ok(user)
        })
        .collect::<Result<_, AppError>>()?;

    Ok(Json(users))
}
