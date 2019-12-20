use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    pub contracted_to: String,
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub signed: bool,
    pub signed_at: Option<DateTime<Utc>>,
    pub to: String,
    pub user_id: String,
}

impl Contract {}

impl Eq for Contract {}

impl Ord for Contract {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialEq for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Contract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for Contract {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}