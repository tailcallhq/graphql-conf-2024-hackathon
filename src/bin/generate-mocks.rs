use std::{fs, sync::{Arc, Mutex}};

use anyhow::Result;
use mock_json::{mock, registry, MockFn};
use serde_json::{json, Value};

const ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Clone)]
struct MockOrderedNumber {
    id: Arc<Mutex<usize>>,
}

impl MockOrderedNumber {
    fn new() -> Self {
        Self {
            id: Arc::new(Mutex::default()),
        }
    }

    fn reset(&self) {
        *self.id.lock().unwrap() = 0;
    }
}

impl MockFn for MockOrderedNumber {
    fn mock(&self, _: Option<Vec<&str>>) -> Value {
        let mut id = self.id.lock().unwrap();

        *id += 1;

        Value::Number((*id).into())
    }
}

fn generate(template: &Value, index: usize) -> Result<()> {
    let value = mock(template);

    fs::write(
        format!("{ROOT_DIR}/mocks/{index}.json"),
        serde_json::to_string_pretty(&value)?,
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let ordered_number_mock = MockOrderedNumber::new();
    registry("@OrderedNumber", ordered_number_mock.clone());

    let template = json!({
        "posts":[{
            "id": "@Number",
            "userId": "@Number|1~10",
            "title": "@Title",
            "body": "@Sentence",
        }, 80, 100],
        "users": [{
            "id": "@OrderedNumber",
            "name": "@Name",
            "username": "@FirstName",
            "email": "@Email",
            "phone": "@Phone",
            "website": "@Url",
            "address": {
                "zipcode": "@Zip",
                "geo": {
                    "lat": "@Float|4|-90~90",
                    "lng": "@Float|4|-180~180",
                }
            }
        }, 9, 10]
    });

    generate(&template, 1)?;
    ordered_number_mock.reset();
	generate(&template, 2)?;
    ordered_number_mock.reset();
    generate(&template, 3)?;

    Ok(())
}
