use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mock_json::mock;
use serde_json::json;

use crate::{AppError, AppState, PostData};

pub async fn handle(
    _state: State<Arc<AppState>>,
    post_id: Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let post_id = post_id.0;
    let template = json!({
        "id": post_id,
        "userId": "@Number|1~10",
        "title": "@Title",
        "body": "@Sentence",
    });

    let mocked_value = mock(&template).to_string();
    let mocked_post_data: PostData = serde_json::from_str(&mocked_value)?;
    Ok(Json(mocked_post_data))
}
