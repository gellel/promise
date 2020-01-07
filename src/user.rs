use chrono::{DateTime, Utc};
use contract::{Contract};
use std::collections::{HashSet};
use uuid::{Uuid};

pub struct User {
    create_at: DateTime<Utc>,
    id: Uuid,
    contract: HashSet<Uuid>,
}

impl User {

    #[allow(dead_code)]
    pub fn add_contract(&mut self, contract: Contract) {}
}