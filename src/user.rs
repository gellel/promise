use chrono::{DateTime, Utc};
use uuid::{Uuid};

pub struct User {
    create_at: DateTime<Utc>,
    id: Uuid,
}