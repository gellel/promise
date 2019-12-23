use crate::promise::{Promise};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};
use std::collections::{HashSet};
use uuid::{Uuid};


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub created_at: DateTime<Utc>,
    pub contracts: Option<HashSet<Uuid>>,
    pub id: Uuid,
    pub name: Option<String>,
    pub promises: Option<HashSet<Uuid>>,
}

impl User {
    pub fn new() -> User {
        User {
            created_at: Utc::now(),
            contracts: Some(HashSet::new()),
            id: Uuid::new_v4(),
            name: None,
            promises: Some(HashSet::new()),
        }
    }
}

impl User {
    pub fn add_promise(&mut self, promise: &Promise) -> bool {
        self.promises.as_mut().unwrap().insert(promise.id)
    }
}

impl Eq for User {}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}