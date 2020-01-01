// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[derive(Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Cx {
    create_at: usize,
    expire_at: isize,
    id: Uuid,
    is_expire: bool,
    is_sign: bool,
    is_sign_from: bool,
    is_sign_to: bool,
    sign_at: isize,
    sign_at_from: isize,
    sign_at_to: isize,
    sign_from: Uuid,
    sign_from_key: String,
    sign_to: Uuid,
    sign_to_key: String,
}

impl Cx {

    /// `new` creates a new `Cx`.
    /// 
    /// `new` relies on the `sign_from` and `sign_to` user having
    /// a valid `RSA_PUBLIC_KEY` that can be shared with the new `Cx`.
    /// 
    /// `new` does not set an expiry for the new `Cx`.
    /// 
    /// # Arguments
    /// * `sign_from` - The user `Uuid` that is being request to sign the `from` half of the contract.
    /// * `sign_to` - The user `Uuid` that is being requested to sign the `to` half of the contract.
    /// 
    /// # Example
    /// An example of creating a new `Cx` with a `sign_from` user and `sign_to` user.
    /// 
    /// The `Cx` handles the `clone` of the `Uuid`.
    /// 
    /// ```let sign_from = Uuid::new_v4();```
    /// ```let sign_to = Uuid::new_v4();```
    /// ```let mut cx = Cx::new(sign_from, sign_to);```
    /// 
    pub fn new(sign_from: Uuid, sign_to: Uuid) -> Cx {
        Cx{
            create_at: 0,
            expire_at: -1,
            id: Uuid::new_v4(),
            is_expire: false,
            is_sign: false,
            is_sign_from: false,
            is_sign_to: false,
            sign_at: -1,
            sign_at_from: -1,
            sign_at_to: -1,
            sign_from: sign_from.clone(),
            sign_from_key: String::from("RSA_PUBLIC_KEY"),
            sign_to: sign_to.clone(),
            sign_to_key: String::from("RSA_PUBLIC_KEY"),
        }
    }
}

impl Cx {
    
    /// `sign_as_from` signs the contract as the `sign_from` user.
    /// 
    /// `sign_as_from` relies on the `sign_from_private_key` being an asymmetric key.
    /// and the asymmetric key being held in its signable state.
    /// 
    /// Returns the `is_sign` state after attempting to sign the `Cx`.
    ///
    /// # Arguments
    /// * `sign_from_private_key` - The stringified private key to verify it is the `sign_from` user.
    /// 
    /// # Example
    /// An example of signing the `Cx` as the `sign_from` user. 
    /// The `RSA_PRIVATE_KEY` must be correspond to the
    /// public key held by the `sign_from` user. If the signature cannot be
    /// verified or the signing fails, the `Cx` is not mutated.
    /// 
    /// ```let mut cx = Cx::default();```
    /// ```let sign_from_private_key = String::from("RSA_PRIVATE_KEY");```
    /// ```cx.sign_as_from(sign_from_private_key);```
    ///
    pub fn sign_as_from(&mut self, sign_from_private_key: String) -> bool {
        self.is_sign_from = true;
        self.sign_at_from = 0;
        self.is_sign = self.is_sign_from == true && self.is_sign_to == true;
        self.is_sign
    }

    /// `sign_as_to` signs the contract as the `sign_to` user.
    /// 
    /// `sign_as_to` relies on the `sign_to_private_key` being an asymmetric key.
    /// and the asymmetric key being held in its signable state.
    /// 
    /// Returns the `is_sign` state after attempting to sign the `Cx`.
    ///
    /// # Arguments
    /// * `sign_to_private_key` - The stringified private key to verify it is the `sign_to` user.
    /// 
    /// # Example
    /// An example of signing the `Cx` as the `sign_to` user. 
    /// The `RSA_PRIVATE_KEY` must be correspond to the
    /// public key held by the `sign_to` user. If the signature cannot be
    /// verified or the signing fails, the `Cx` is not mutated.
    /// 
    /// ```let mut cx = Cx::default();```
    /// ```let sign_to_private_key = String::from("RSA_PRIVATE_KEY");```
    /// ```Cx.sign_as_to(sign_to_private_key);```
    ///
    pub fn sign_as_to(&mut self, sign_to_private_key: String) -> bool {
        self.is_sign_to = true;
        self.sign_at_to = 0;
        self.is_sign = self.is_sign_from == true && self.is_sign_to == true;
        self.is_sign
    }
}