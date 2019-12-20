use chrono::{DateTime, Utc};
use std::collections::{HashSet};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub name: String,
    pub promises_broken: u128,
    pub promises_from: Option<HashSet<String>>,
    pub promises_kept: u128,
    pub promises_to: Option<HashSet<String>>
}

impl User {

    pub fn break_promise(&self, id: &String) {

    }

    pub fn keep_promise(&self, id: &String) {
        
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    } 
}

impl ToString for User {

    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}