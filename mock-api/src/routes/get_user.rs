use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mock_json::mock;
use serde_json::json;

use crate::{AppError, AppState, UserData};

pub async fn handle(
    _state: State<Arc<AppState>>,
    user_id: Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_id.0;
    let template = json!({
            "id": user_id,
            "name": "@Name",
            "username": "@FirstName",
            "email": "@Email",
            "phone": "@Phone",
            "website": "@Url",
            "address": {
                "zipcode": "@Zip",
                "geo": {
                    "lat": "@Float",
                    "lon": "@Float",
                }
            }
    });
    let mocked_value = mock(&template).to_string();
    let mocked_user_data: UserData = serde_json::from_str(&mocked_value)?;
    Ok(Json(mocked_user_data))
}
