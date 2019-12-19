use chrono::{DateTime, Utc};
use std::collections::{HashSet};

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct User {
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub name: mut String,
}

impl ToString for User {

    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}