use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};
use uuid::{Uuid};

#[derive(Debug, Deserialize, Eq, Hash, Serialize)]
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

    #[allow(dead_code)]
    fn set_sign_at(&mut self, utc_sign_at: DateTime<Utc>) -> bool {
        self.is_sign = self.is_sign_from == true && self.is_sign_to == true;
        if self.is_sign {
            self.sign_at = Some(utc_sign_at.clone());
        }
        return self.is_sign;
    }

    #[allow(dead_code)]
    fn set_sign_at_from(&mut self, utc_sign_at_from: DateTime<Utc>) -> bool {
        let utc_now = Utc::now();
        self.sign_at_from = Some(utc_sign_at_from.clone());
        return utc_now.lt(&utc_sign_at_from);
    }

    #[allow(dead_code)]
    fn set_sign_at_to(&mut self, utc_sign_at_to: DateTime<Utc>) -> bool {
        let utc_now = Utc::now();
        self.sign_at_from = Some(utc_sign_at_to.clone());
        return utc_now.lt(&utc_sign_at_to);
    }
}

impl Cx {

    /// `is_current` asserts if the `&mut self`
    /// has not passed its permitted signature timestamp.
    #[allow(dead_code)]
    pub fn is_current(&mut self) -> bool {
        let utc_now = Utc::now();
        match &mut self.expire_at {
            None => true,
            Some(utc_expire) => utc_now.lt(utc_expire)
        }
    }

    /// `is_not_current` asserts if the `&mut self`
    /// has passed its permitted signature timestamp.
    #[allow(dead_code)]
    pub fn is_not_current(&mut self) -> bool {
        return self.is_current() == false
    }

    /// `can_sign_from` asserts if the `&mut self` 
    /// can be cryptographically signed as the `&mut self.sign_from`.
    #[allow(dead_code)]
    pub fn can_sign_from(&mut self) -> bool {
        return self.is_current() && self.sign_at_from.is_none();
    }

    /// `can_sign_to` asserts if the `&mut self` 
    /// can be cryptographically signed as the `&mut self.sign_from`.
    #[allow(dead_code)]
    pub fn can_sign_to(&mut self) -> bool {
        return self.is_current() && self.sign_at_to.is_none();
    }

    /// `can_not_sign_from` asserts if the `&mut self` 
    /// cannot be cryptographically signed as the `&mut self.sign_from`.
    #[allow(dead_code)]
    pub fn can_not_sign_from(&mut self) -> bool {
        return self.can_sign_from() == false;
    }

    /// `can_not_sign_to` asserts if the `&mut self` 
    /// cannot be cryptographically signed as the `&mut self.sign_to`.
    #[allow(dead_code)]
    pub fn can_not_sign_to(&mut self) -> bool {
        return self.can_sign_to() == false;
    }
    
    /// `sign_as_from` cryptographically signs the `&mut self`
    /// as the `&mut self.sign_from`.
    /// 
    /// uses the `&mut self.sign_from_key` to validate the
    /// incoming signature.
    #[allow(dead_code)]
    pub fn sign_as_from(&mut self, _: String) -> bool {
        let utc_now = Utc::now();
        if self.can_not_sign_to() {
            return self.is_sign_from;
        }
        self.is_sign_from = true;
        self.set_sign_at_from(utc_now);
        self.set_sign_at(utc_now);
        return self.is_sign_from;
    }

    /// `sign_as_to` cryptographically signs the `&mut self`
    /// as the `&mut self.sign_to`.
    /// 
    /// uses the `&mut self.sign_to_key` to validate the
    /// incoming signature.
    #[allow(dead_code)]
    pub fn sign_as_to(&mut self, _: String) -> bool {
        let utc_now = Utc::now();
        if self.is_not_current() {
            return self.is_sign_to;
        }
        self.is_sign_to = true;
        self.set_sign_at_to(utc_now);
        self.set_sign_at(utc_now);
        return self.is_sign_to;
    }
}

impl Ord for Cx {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.create_at.cmp(&other.create_at);
    }
}

impl PartialEq for Cx {
    fn eq(&self, other: &Self) -> bool {
        return self.create_at == other.create_at;
    }
}

impl PartialOrd for Cx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
