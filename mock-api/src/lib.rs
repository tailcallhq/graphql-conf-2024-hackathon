use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub mod routes;

#[derive(Default)]
pub struct AppState {
    pub users: HashMap<i64, UserData>,
    pub posts: HashMap<String, PostData>,
}

#[derive(Serialize, Clone)]
pub struct UserData {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub username: String,
    pub website: String,
    pub address: AddressData,
}

#[derive(Clone, Serialize)]
pub struct AddressData {
    pub geo: GeoData,
    pub zipcode: String,
}

#[derive(Clone, Serialize)]
pub struct GeoData {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Clone, Serialize)]
pub struct PostData {
    pub id: String,
    pub title: String,
    pub user_id: i64,
    pub body: String,
}

#[allow(dead_code)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("INTERNAL_SERVER_ERROR"),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
