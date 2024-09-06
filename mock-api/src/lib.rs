use std::{collections::HashMap, sync::Mutex};

use axum::{http::StatusCode, response::IntoResponse};
use mock_json::mock;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod routes;

struct Database {
    user_template: serde_json::Value,
    post_template: serde_json::Value,
    users: Mutex<HashMap<i64, UserData>>,
    posts: Mutex<HashMap<i64, PostData>>,
}

impl Database {
    pub fn new() -> Self {
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
            users: Mutex::new(HashMap::new()),
            posts: Mutex::new(HashMap::new()),
        }
    }

    pub fn update(&self) -> Result<(), anyhow::Error> {
        self.reset();

        // Generate and store users
        let mut users_map = self.users.lock().unwrap();
        (1..=10).for_each(|id| {
            let mut user: UserData =
                serde_json::from_str(&mock(&self.user_template).to_string()).unwrap();
            user.id = id;
            users_map.insert(id, user);
        });

        // Generate and store posts
        let mut posts_map = self.posts.lock().unwrap();
        (1..=100).for_each(|id| {
            let mut post: PostData =
                serde_json::from_str(&mock(&self.post_template).to_string()).unwrap();
            post.id = id;
            posts_map.insert(id, post);
        });

        Ok(())
    }

    pub fn posts(&self) -> Vec<PostData> {
        self.posts.lock().unwrap().values().cloned().collect()
    }

    pub fn user(&self, id: i64) -> Option<UserData> {
        self.users.lock().unwrap().get(&id).cloned()
    }

    pub fn post(&self, id: i64) -> Option<PostData> {
        self.posts.lock().unwrap().get(&id).cloned()
    }

    pub fn users(&self) -> Vec<UserData> {
        self.users.lock().unwrap().values().cloned().collect()
    }

    pub fn reset(&self) {
        self.users.lock().unwrap().clear();
        self.posts.lock().unwrap().clear();
    }
}

pub struct AppState {
    pub db: Database,
}

impl Default for AppState {
    fn default() -> Self {
        let db = Database::new();
        db.update();

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
