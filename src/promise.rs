use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};

trait Break {
    fn break_promise(&mut self);
}

trait Complete {
    fn complete_promise(&mut self);
}

trait End {
    fn end_promise(&mut self);
}

trait Kept {
    fn kept_promise(&mut self);
}

trait Start {
    fn start_promise(&mut self);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Promise {
    pub actions: Option<HashSet<String>>,
    pub broken: bool,
    pub category: String,
    pub complete: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub contracts: Option<HashSet<String>>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub end_at: Option<DateTime<Utc>>,
    pub id: String,
    pub kept: bool,
    pub labels: Option<HashSet<String>>,
    pub name: String,
    pub start_at: Option<DateTime<Utc>>,
    pub subcategory: Option<String>,
    pub to: Option<HashSet<String>>,
    pub user_id: String,
}

impl Break for Promise {
    fn break_promise(&mut self) {
        self.broken = true;
    }
}

impl Complete for Promise {
    fn complete_promise(&mut self) {
        self.completed_at = Some(Utc::now());
    }
}

impl PartialEq for Promise {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    } 
}

impl ToString for Promise {

    fn to_string(&self) -> String {
       serde_json::to_string(&self).unwrap()
    }
}