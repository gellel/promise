use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};
use std::collections::{HashSet};
use uuid::{Uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct Promise {
    pub actions: Option<HashSet<Uuid>>,
    pub broken: Option<bool>,
    pub categories: Option<HashSet<String>>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub contracts: Option<HashSet<Uuid>>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub end_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub kept: Option<bool>,
    pub labels: Option<HashSet<String>>,
    pub name: Option<String>,
    pub start_at: Option<DateTime<Utc>>,
    pub subcategories: Option<HashSet<String>>,
    pub to: Option<HashSet<Uuid>>,
    pub user_id: Uuid,
}

impl Promise {}

impl Eq for Promise {}

impl Ord for Promise {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialEq for Promise {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Promise {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for Promise {
    fn to_string(&self) -> String {
       serde_json::to_string(&self).unwrap()
    }
}