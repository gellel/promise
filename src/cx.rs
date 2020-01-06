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
    ref_id: Uuid,
    sign_at: Option<DateTime<Utc>>,
    sign_at_from: Option<DateTime<Utc>>,
    sign_at_to: Option<DateTime<Utc>>,
    sign_from: Uuid,
    sign_from_key: String,
    sign_to: Uuid,
    sign_to_key: String,
}

impl Cx {

    /// `new` creates a new `Cx`.
    /// 
    /// `new` requires three unique `Uuid` to compose a new `Cx`. 
    /// 
    /// `new` does not take ownership of `ref_id`, `sign_from` or `sign_to`.
    /// 
    /// `ref_id` is the `Uuid` of the `struct` the `Cx` is owned by.  
    /// 
    /// `sign_from` is the `Uuid` of the user that is signing from. 
    /// 
    /// `sign_to` is the `Uuid` of the user that is signing to. 
    #[allow(dead_code)]
    pub fn new(ref_id: Uuid, sign_from: Uuid, sign_to: Uuid) -> Cx {
        Cx{
            create_at: Utc::now(),
            expire_at: None,
            id: Uuid::new_v4(),
            is_sign: false,
            is_sign_from: false,
            is_sign_to: false,
            ref_id: ref_id.clone(),
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

    /// `&mut self.set_sign_at` sets `&mut self.sign_at` to the
    /// argument `utc_sign_at`.
    /// 
    /// `&mut self.set_sign_at` returns a `bool` indicating whether the
    /// argument `utc_sign_at` successfully mutated `&mut self.is_sign`
    /// and `&mut self.sign_at`. 
    /// 
    /// `&mut self.set_sign_at` can only mutate `&mut self.is_sign` and `&mut self.sign_at`
    /// if both `&mut self.is_sign_from` && `mut self.is_sign_to` are
    /// both `true`.
    /// 
    /// `&mut self.set_sign_at` does not take ownership of `utc_sign_at`.
    #[allow(dead_code)]
    fn set_sign_at(&mut self, utc_sign_at: DateTime<Utc>) -> bool {
        self.is_sign = self.is_sign_from == true && self.is_sign_to == true;
        if self.is_sign {
            self.sign_at = Some(utc_sign_at.clone());
        }
        return self.is_sign;
    }
    
    /// `&mut self.set_sign_at_from` sets the `&mut self.sign_at_from` 
    /// to the argument `utc_sign_at_from`.
    /// 
    /// `&mut self.set_sign_at_from` returns a `bool` indicating whether the
    /// argument `utc_sign_at_from` successfully mutated `&mut self.sign_at_to`.
    /// 
    /// `&mut self.set_sign_at_from` is only modified if the argument `utc_sign_at_from`
    /// is earlier than the current UTC time.
    /// 
    /// `&mut self.set_sign_at_from` does not take ownership of `utc_sign_at_from`.
    #[allow(dead_code)]
    fn set_sign_at_from(&mut self, utc_sign_at_from: DateTime<Utc>) -> bool {
        let utc_now = Utc::now();
        let is_earlier = utc_now.gt(&utc_sign_at_from);
        if !is_earlier {
            return is_earlier;
        }
        self.sign_at_from = Some(utc_sign_at_from.clone());
        return is_earlier;
    }

    /// `&mut self.set_sign_at_to` sets the `&mut self.sign_at_to`
    /// to the argument `utc_sign_at_to`.
    /// 
    /// `&mut self.set_sign_at_to` returns a `bool` indicating whether the
    /// argument `utc_sign_at_to` successfully mutated `&mut self.sign_at_to`.
    /// 
    /// `&mut self.set_sign_at_to` is only modified if the argument `utc_sign_at_to`
    /// is earlier than the current UTC time.
    #[allow(dead_code)]
    fn set_sign_at_to(&mut self, utc_sign_at_to: DateTime<Utc>) -> bool {
        let utc_now = Utc::now();
        let is_earlier = utc_now.gt(&utc_sign_at_to);
        if !is_earlier {
            return is_earlier;
        }
        self.sign_at_to = Some(utc_sign_at_to.clone());
        return is_earlier;
    }
}

impl Cx {

    /// `&mut self.is_current` asserts that `&mut self` has not exceed its expiry. 
    /// 
    /// `&mut self.is_current` is considered current on the condition 
    /// `&mut self.expire_at` is either `None` or `&mut self.expire_at` is `gt`
    /// the current UTC timestamp. 
    #[allow(dead_code)]
    pub fn is_current(&mut self) -> bool {
        let utc_now = Utc::now();
        match &mut self.expire_at {
            None => true,
            Some(utc_expire) => utc_now.lt(utc_expire)
        }
    }

    /// `&mut self.is_not_current` asserts that `&mut self` has execeeded its expiry.
    /// 
    /// `&mut self.is_not_current` is considered not current on the condition
    /// `&mut self.is_current` is equal to `false`. 
    #[allow(dead_code)]
    pub fn is_not_current(&mut self) -> bool {
        return self.is_current() == false
    }

    /// `&mut self.can_sign_from` asserts that `&mut self` can be cryptographically signed
    /// as `&mut self.sign_from`. 
    /// 
    /// `&mut self.can_sign_from` can perform a signature on the condition that
    /// `&mut self.is_current` is equal to `true` and `&mut self.sign_at_from` is `None`. 
    /// 
    /// `&mut self.sign_at_from` is required to be `None` as `&mut self` is only
    /// permitted to be signed once as `&mut self.sign_from`.  
    #[allow(dead_code)]
    pub fn can_sign_from(&mut self) -> bool {
        return self.is_current() && self.sign_at_from.is_none();
    }

    /// `&mut self.can_sign_to` asserts that `&mut self` can be cryptographically signed
    /// as `&mut self.sign_to`. 
    /// 
    /// `&mut self.can_sign_to` can perform a signature on the condition that
    /// `&mut self.is_current` is equal to `true` and `&mut self.sign_at_to` is `None`. 
    /// 
    /// `&mut self.sign_at_to` is required to be `None` as `&mut self` is only
    /// permitted to be signed once as `&mut self.sign_to`.  
    #[allow(dead_code)]
    pub fn can_sign_to(&mut self) -> bool {
        return self.is_current() && self.sign_at_to.is_none();
    }

    /// `&mut self.can_not_sign_from` asserts that `&mut self` cannot be cryptographically signed
    /// as `&mut self.sign_from`.
    /// 
    /// `&mut self.can_not_sign_from` cannot perform a signature on the condition
    /// `&mut self.can_sign_from` is equal to `false`. 
    #[allow(dead_code)]
    pub fn can_not_sign_from(&mut self) -> bool {
        return self.can_sign_from() == false;
    }

    /// `&mut self.can_not_sign_to` asserts that `&mut self` cannot be cryptographically signed
    /// as `&mut self.sign_to`.
    /// 
    /// `&mut self.can_not_sign_to` cannot perform a signature on the condition
    /// `&mut self.can_sign_to` is equal to `false`. 
    #[allow(dead_code)]
    pub fn can_not_sign_to(&mut self) -> bool {
        return self.can_sign_to() == false;
    }
    
    /// `&mut self.sign_as_from` cryptographically signs the `&mut self`
    /// as the `&mut self.sign_from`.
    /// 
    /// `&mut self.sign_as_from` uses the `&mut self.sign_from_key` to validate the
    /// incoming signature.
    /// 
    /// `&mut self.sign_as_from` can return `false` on two cases.
    /// 
    /// The first case is where `&mut self` contains a non `None`
    /// `&mut self.expire_at`. If `&mut self.expire_at` is not `None`
    /// and the current UTC is later than `&mut self.expire_at` 
    /// `&mut self.sign_as_from` returns `false` and does not update `&mut self`.
    /// 
    /// The last case is where `&mut self` cannot use the argument
    /// `sign_as_from_key`. If `&mut self` cannot use the `sign_as_from_key`
    /// `&mut self.sign_as_from` returns `false` and does not update `&mut self`.
    /// 
    /// On success returns `true` and `&mut self` is appropriately modified.
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

    /// `&mut self.sign_as_to` cryptographically signs the `&mut self`
    /// as the `&mut self.sign_to`.
    /// 
    /// `&mut self.sign_as_to` uses the `&mut self.sign_to_key` to validate the
    /// incoming signature.
    /// 
    /// `&mut self.sign_as_to` can return `false` on two cases.
    /// 
    /// The first case is where `&mut self` contains a non `None`
    /// `&mut self.expire_at`. If `&mut self.expire_at` is not `None`
    /// and the current UTC is later than `&mut self.expire_at` 
    /// `&mut self.sign_as_to` returns `false` and does not update `&mut self`.
    /// 
    /// The last case is where `&mut self` cannot use the argument
    /// `sign_as_to_key`. If `&mut self` cannot use the `sign_as_to_key`
    /// `&mut self.sign_as_to` returns `false` and does not update `&mut self`.
    /// 
    /// On success returns `true` and `&mut self` is appropriately modified.
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
