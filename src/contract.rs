use crate::user::{User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering};
use uuid::{Uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    pub contracted_to: Uuid,
    pub created_at: DateTime<Utc>,
    pub id: Uuid,
    pub signed: bool,
    pub signed_at: Option<DateTime<Utc>>,
    pub to: Uuid,
    pub user_id: Uuid,
}

impl Contract {
    pub fn new(contracted_to: Uuid, user_from: User, user_to: User) -> Contract {
        Contract {
            contracted_to: contracted_to,
            created_at: Utc::now(),
            id: Uuid::new_v4(),
            signed: false,
            signed_at: None,
            to: user_to.id,
            user_id: user_from.id,
        }
    }
}

impl Eq for Contract {}

impl Ord for Contract {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

impl PartialEq for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Contract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for Contract {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}