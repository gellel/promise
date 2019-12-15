extern crate chrono;
use chrono::{DateTime, Utc};

pub struct User {
    pub created_at: DateTime<Utc>,
    pub id: String,
}

impl User {
    pub fn new(id: String) -> User {
        return User{
            created_at: Utc::now(),
            id: id
        };
    }
}