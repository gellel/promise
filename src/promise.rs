use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Promise {
    pub category: String,
    pub complete: bool,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub end_at: Option<DateTime<Utc>>,
    pub id: String,
    pub name: String,
    pub start_at: Option<DateTime<Utc>>,
}