mod contract;

use chrono::{DateTime, Utc};
use contract::{Contract};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};

#[derive(Debug, Deserialize, Serialized)]
pub struct Task {
    pub category: String,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub contracts: Option<mut HashSet<String>>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub end_at: Option<DateTime<Utc>>,
    pub labels: Option<mut HashSet<String>>,
    pub rating: u8,
    pub subcategory: String,
    pub started: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub user_id: String,
}

impl ToString for Task {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
    }
}