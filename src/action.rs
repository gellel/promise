use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
    pub description: String,
    pub id: String,
    pub labels: Option<HashSet<String>>,
    pub name: String,
    pub subcategory: String,
    pub tasks: Option<HashSet<String>>,
    pub user_id: String,
}

impl ToString for Action {

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}