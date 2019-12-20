use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub description: String,
    pub id: String,
    pub labels: Option<mut HashSet<String>>,
    pub name: String,
    pub subcategory: String,
    pub tasks: Option<mut HashSet<String>>,
    pub user_id: String,
}

impl Action {

    pub fn complete_action(&mut self) {
        self.completed = true;
        self.completed_at = Utc::now();
    }
}

impl PartialEq for Action {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl ToString for Action {

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}