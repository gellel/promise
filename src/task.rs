mod contract;

use chrono::{DateTime, Utc};
use contract::{Contract};
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};
use std::collections::{HashSet};
use uuid::{Uuid};

#[derive(Debug, Deserialize, Serialized)]
pub struct Task {
    pub categories: Option<HashSet<String>>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub contracts: Option<HashSet<String>>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub end_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub labels: Option<HashSet<String>>,
    pub rating: Option<u8>,
    pub subcategories: Option<HashSet<String>>,
    pub started: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub user_id: String,
}

impl Eq for Task {}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
    }
}