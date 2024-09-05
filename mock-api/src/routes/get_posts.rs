use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use mock_json::mock;

use crate::{AppError, AppState, PostData};

pub async fn handle(state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let posts: Vec<PostData> = (1..=10)
        .map(|id| {
            let mut post: PostData = serde_json::from_str(&mock(&state.post_template).to_string())?;
            post.id = id;
            Ok(post)
        })
        .collect::<Result<_, AppError>>()?;

    Ok(Json(posts))
}
