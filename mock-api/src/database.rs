use std::{collections::HashMap, sync::Mutex};

use mock_json::mock;
use serde_json::json;

use crate::{PostData, UserData};

pub struct Database {
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

    pub fn reset(&self) -> Result<(), anyhow::Error> {
        // clear the previous data from database.
        self.users
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to clear users."))?
            .clear();
        self.posts
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to clear posts."))?
            .clear();

        // Generate and store users
        let mut users_map = self
            .users
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to access users"))?;
        for id in 1..=10 {
            let mut user: UserData = serde_json::from_value(mock(&self.user_template))
                .map_err(|_| anyhow::anyhow!("Failed to generate user"))?;
            user.id = id;
            users_map.insert(id, user);
        }

        // Generate and store posts
        let mut posts_map = self
            .posts
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to access posts"))?;
        for id in 1..=20 {
            let mut post: PostData = serde_json::from_value(mock(&self.post_template))
                .map_err(|_| anyhow::anyhow!("Failed to generate user"))?;
            post.id = id;
            posts_map.insert(id, post);
        }

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
}
