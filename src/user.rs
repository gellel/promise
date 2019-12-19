use chrono::{DateTime, Utc};
use std::collections::{HashSet};

pub struct User {
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub name: mut String,
    pub promises: Option<mut HashSet<String>>,
}