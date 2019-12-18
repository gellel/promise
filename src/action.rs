use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

pub struct Action {
    created_at: DateTime<Utc>,
    id: String,
}

impl Action {

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}