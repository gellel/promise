use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub signed: bool,
    pub signed_at: Option<DateTime<Utc>>,
    pub to: String,
    pub user_id: String,
}

impl ToString for Contract {

    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}