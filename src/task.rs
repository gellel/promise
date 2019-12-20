mod contract;

use chrono::{DateTime, Utc};
use contract::{Contract};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};


pub struct Task {
    pub category: String,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub contracts: Option<mut HashSet<String>>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub end_at: Option<DateTime<Utc>>,
    pub labels: Option<mut HashSet<String>>,
    pub rating: u128,
    pub subcategory: String,
    pub started: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub user_id: String,
}

impl Task {

    pub fn end_task(&mut self) -> {
        self.completed_at = Some(Utc::now())
        self.completed = true;
    }
    
    pub fn start_task(&mut self) -> bool {
        self.started_at = Some(Utc::now())
        self.started = true
    }
}

impl Task {

    pub fn add_contract(&mut self, contract: &Contract) -> bool {
        if contract.signed { self.contracts.insert(contract.id) } else { false }
    }

    pub fn del_contract(&mut self, contract: &Contract) -> bool {
        self.contracts.delete(contract.id)
    }
}