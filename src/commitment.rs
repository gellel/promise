use chrono::{DateTime, Utc};
use std::fmt;
use uuid::{Uuid};

#[derive(Debug)]
pub struct Commitment<'a> {
    category: &'a str,
    created: DateTime<Utc>,
    description: &'a str,
    end_date: Option<DateTime<Utc>>,
    expired: bool,
    from: &'a str,
    id: Uuid,
    name: &'a str,
    start_date: Option<DateTime<Utc>>,
    timed: bool,
    to: &'a str,
}

impl<'a> Commitment<'a> {

}

impl fmt::Display for Commitment<'_> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
