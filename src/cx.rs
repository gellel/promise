// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[derive(Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Cx {
    create_at: usize,
    id: Uuid,
    is_expire: bool,
    is_sign: bool,
    is_sign_from: bool,
    is_sign_to: bool,
    sign_at: usize,
    sign_at_from: usize,
    sign_at_to: usize,
    sign_from: Uuid,
    sign_from_key: String,
    sign_to: Uuid,
    sign_to_key: String,
}

impl Cx {

    /// `sign_as_from` signs the contract as the `sign_from`.
    /// 
    /// `sign_as_from` relies on the `sign_from_private_key` being an asymmetric key.
    ///
    /// # Arguments
    /// * `sign_from_private_key` - The stringified private key to verify it is the `sign_from` user.
    /// 
    /// # Example
    /// // cx.sign_as_from(String:from(""));
    /// 
    pub fn sign_as_from(&mut self, sign_from_private_key: String) {
        self.is_sign_from = true;
        self.sign_at_from = 0;
    }

    /// `sign_as_to` signs the contract as the `sign_to`.
    /// 
    /// `sign_as_to` relies on the `sign_from_private_key` being an asymmetric key.
    ///
    /// # Arguments
    /// * `sign_from_private_key` - The stringified private key to verify it is the `sign_from` user.
    /// 
    /// # Example
    /// // cx.sign_as_to(String:from(""));
    /// 
    pub fn sign_as_to(&mut self, sign_to_private_key: String) {
        self.is_sign_to = true;
        self.sign_at_to = 0;
    }
}