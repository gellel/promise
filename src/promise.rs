use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};

#[derive(Debug, Deserialize, Serialize)]
pub struct Promise {
    pub actions: Option<HashSet<String>>,
    pub category: String,
    pub complete: bool,
    pub contracts: Option<HashSet<String>>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub end_at: Option<DateTime<Utc>>,
    pub id: String,
    pub labels: Option<HashSet<String>>,
    pub name: String,
    pub start_at: Option<DateTime<Utc>>,
    pub subcategory: Option<String>,
    pub to: HashSet<String>,
    pub user_id: String,
}

impl ToString for Promise {

    fn to_string(&self) -> String {
       serde_json::to_string(&self).unwrap()
    }
}

impl PartialEq for Promise {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    } 
}