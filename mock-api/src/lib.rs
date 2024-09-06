use axum::{http::StatusCode, response::IntoResponse};
use database::Database;
use serde::{Deserialize, Serialize};

pub mod database;
pub mod routes;

pub struct AppState {
    pub db: Database,
}

impl Default for AppState {
    fn default() -> Self {
        let db = Database::new();
        let _ = db.update().unwrap();
        Self { db }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserData {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub username: String,
    pub website: String,
    pub address: AddressData,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AddressData {
    pub geo: GeoData,
    pub zipcode: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GeoData {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostData {
    pub id: i64,
    pub title: String,
    pub user_id: i64,
    pub body: String,
}

pub enum AppError {
    NotFound(String),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}

