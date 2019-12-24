use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};
use std::collections::{HashSet};
use uuid::{Uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct Promise {
    pub actions: Option<HashSet<Uuid>>,
    pub categories: Option<HashSet<String>>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub contracts: Option<HashSet<Uuid>>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub end_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub is_broken: Option<bool>,
    pub is_kept: Option<bool>,
    pub labels: Option<HashSet<String>>,
    pub name: Option<String>,
    pub start_at: Option<DateTime<Utc>>,
    pub subcategories: Option<HashSet<String>>,
    pub to: Option<HashSet<Uuid>>,
    pub user_id: Uuid,
}

impl Promise {
    pub fn new(user_id: Uuid) -> Promise {
        Promise {
            actions: Some(HashSet::new()),
            categories: Some(HashSet::new()),
            completed: false,
            completed_at: None,
            contracts: Some(HashSet::new()),
            created_at: Utc::now(),
            description: None,
            end_at: None,
            ended_at: None,
            id: Uuid::new_v4(),
            is_broken: None,
            is_kept: None,
            labels: Some(HashSet::new()),
            name: None,
            start_at: None,
            subcategories: Some(HashSet::new()),
            to: Some(HashSet::new()),
            user_id: user_id,
        }
    }
}

impl Promise {
    pub fn insert_action(&mut self, action_id: Uuid) -> bool {
        self.actions.as_mut().unwrap().insert(action_id);
    }
}

impl Promise {
    pub fn insert_to(&mut self, user_id: Uuid) -> bool {
        self.to.as_mut().unwrap().insert(user_id)
    }
}

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