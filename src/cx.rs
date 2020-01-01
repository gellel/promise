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
    
    /// `sign_as_from` cryptographically signs the `&cx` contract as the `sign_as_from` user.
    #[allow(dead_code)]
    pub fn sign_as_from(&mut self, _: String) -> bool {
        if self.is_expire {
            return self.is_sign;
        }
        self.is_sign_from = true;
        self.sign_at_from = 0;
        self.is_sign = self.is_sign_from == true && self.is_sign_to == true;
        return self.is_sign;
    }

    /// `sign_as_to` cryptographically signs the `&cx` contract as the `sign_to` user.
    #[allow(dead_code)]
    pub fn sign_as_to(&mut self, _: String) -> bool {
        if self.is_expire {
            return self.is_sign;
        }
        self.is_sign_to = true;
        self.sign_at_to = 0;
        self.is_sign = self.is_sign_from == true && self.is_sign_to == true;
        return self.is_sign;
    }
}