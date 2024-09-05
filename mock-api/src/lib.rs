use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod routes;

pub struct AppState {
    // pub users: HashMap<i64, UserData>,
    // pub posts: HashMap<String, PostData>,
    pub user_template: serde_json::Value,
    pub post_template: serde_json::Value,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            user_template: json!({
                    "id": "@Number|1~10",
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
            }),
            post_template: json!({
                "id": "@Number|1~10",
                "userId": "@Number|1~10",
                "title": "@Title",
                "body": "@Sentence",
            }),
        }
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
