use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Cx {
    create_at: DateTime<Utc>,
    expire_at: Option<DateTime<Utc>>,
    id: Uuid,
    is_sign: bool,
    is_sign_from: bool,
    is_sign_to: bool,
    sign_at: Option<DateTime<Utc>>,
    sign_at_from: Option<DateTime<Utc>>,
    sign_at_to: Option<DateTime<Utc>>,
    sign_from: Uuid,
    sign_from_key: String,
    sign_to: Uuid,
    sign_to_key: String,
}

impl Cx {

    /// `new` creates new `Cx`.
    /// 
    /// # Arguments
    /// 
    /// * `sign_from` - The `uuid::Uuid` of the user to `clone` into the `&mut self.sign_from` field.
    /// 
    /// * `sign_to` - The `uuid::Uuid` of the user to `clone` into the `&mut self.sign_to` field.
    /// 
    /// # Returns
    /// * `Cx{}` - The initialized `Cx`.
    ///
    /// # Example
    /// 
    /// Creating a new `&Cx{}`
    ///
    /// ```
    /// use cx::Cx;
    /// use uuid::Uuid;
    ///
    /// let sign_from = Uuid::new_v4();
    /// let sign_to = Uuid::new_v4();
    /// 
    /// let mut cx = Cx::new(sign_from, sign_to);
    /// 
    /// println!("{:?}", cx);
    /// ```
    ///
    #[allow(dead_code)]
    pub fn new(sign_from: Uuid, sign_to: Uuid) -> Cx {
        Cx{
            create_at: Utc::now(),
            expire_at: None,
            id: Uuid::new_v4(),
            is_sign: false,
            is_sign_from: false,
            is_sign_to: false,
            sign_at: None,
            sign_at_from: None,
            sign_at_to: None,
            sign_from: sign_from.clone(),
            sign_from_key: String::from(""),
            sign_to: sign_to.clone(),
            sign_to_key: String::from(""),
        }
    }
}

impl Cx {

    pub fn is_current(&mut self) -> bool {
        let utc_now = Utc::now();
        match &self.expire_at {
            None => false,
            Some(utc_expire) => utc_now.lt(utc_expire)
        }
    }

    pub fn is_not_current(&mut self) -> bool {
        return self.is_current() == false
    }

    /// `can_sign_from` validates the `&cx` can be signed as the `sign_from` user.
    #[allow(dead_code)]
    pub fn can_sign_from(&mut self) -> bool {
        return false;
    }
    
    /// `sign_as_from` cryptographically signs the `&cx` contract as the `sign_from` user.
    #[allow(dead_code)]
    pub fn sign_as_from(&mut self, _: String) -> bool {
        let utc_now = Utc::now();
        if self.is_not_current() {
            return self.is_sign_from;
        }
        self.is_sign_from = true;
        self.sign_at_from = Some(utc_now.clone());
        self.is_sign = self.is_sign_from == true && self.is_sign_to == true;
        if self.is_sign {
            self.sign_at = Some(utc_now.clone());
        }
        return self.is_sign_from;
    }

    /// `sign_as_to` cryptographically signs the `&cx` contract as the `sign_to` user.
    #[allow(dead_code)]
    pub fn sign_as_to(&mut self, _: String) -> bool {
        let utc_now = Utc::now();
        if self.is_not_current() {
            return self.is_sign_to;
        }
        self.is_sign_to = true;
        self.sign_at_to = Some(utc_now);
        self.is_sign = self.is_sign_from == true && self.is_sign_to == true;
        if self.is_sign {
            self.sign_at = Some(utc_now.clone());
        }
        return self.is_sign_to;
    }
}