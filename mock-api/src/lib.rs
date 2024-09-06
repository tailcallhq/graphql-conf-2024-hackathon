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
