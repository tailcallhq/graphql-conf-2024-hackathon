use axum::{http::StatusCode, response::IntoResponse};
use database::Database;
use serde::{Deserialize, Serialize};

pub mod database;
pub mod routes;
pub mod utils;

/// Represents the application state
pub struct AppState {
    pub db: Database,
}

impl Default for AppState {
    fn default() -> Self {
        let db = Database::new();
        db.reset().unwrap();
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
    pub lng: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostData {
    pub id: i64,
    pub title: String,
    pub user_id: i64,
    pub body: String,
}

/// Custom error types for the application.
pub enum AppError {
    /// Error indicating that a requested resource was not found.
    NotFound(String),
    /// Error indicating an internal server error occurred.
    InternalServerError(String),
}

impl IntoResponse for AppError {
    /// Convert the error into an HTTP response.
    ///
    /// Maps the error variant to the corresponding HTTP status code and message.
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            AppError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}
