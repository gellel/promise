use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};
use std::collections::{HashSet};
use uuid::{Uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub categories: Option<HashSet<String>>,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub contracts: Option<HashSet<String>>,
    pub description: Option<String>,
    pub id: Uuid,
    pub labels: Option<HashSet<String>>,
    pub name: Option<String>,
    pub subcategories: Option<HashSet<String>>,
    pub tasks: Option<HashSet<String>>,
    pub user_id: String,
}

impl Action {}

impl Eq for Action {}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}