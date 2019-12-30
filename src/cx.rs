use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Cx<T> {
    create_at: usize,
    expire_at: Option<usize>,
    from_key: T,
    is_complete: bool,
    is_ephemeral: Option<bool>,
    is_expire: bool,
    is_sign: bool,
    is_symmetrical: bool,
    owner: Uuid,
    sign_at: usize,
    sign_from: Uuid,
    sign_to: Uuid,
    to_key: T,
    uuid: Uuid,
}

impl<T> Cx<T> {
}
impl<T> Cx<T> {
    pub fn set_sign(&mut self, sign_from: Uuid, sign_to: Uuid) {
        self.set_sign_from(sign_from);
        self.set_sign_to(sign_to);
    }
    pub fn set_sign_from(&mut self, sign_from: Uuid) {
        self.sign_from = sign_from;
    }
    pub fn set_sign_to(&ut self, sign_to: Uuid) {
        self.sign_to = sign_to;
    }
}
