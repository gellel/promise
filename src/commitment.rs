use chrono::{DateTime, Utc};
use std::fmt;
use std::collections::HashSet;
use uuid::{Uuid};

#[derive(Debug)]
pub struct Commitment<'a> {
    category: &'a str,
    created: DateTime<Utc>,
    description: &'a str,
    end_date: DateTime<Utc>,
    expired: bool,
    from: &'a usize,
    id: &'a str,
    name: &'a str,
    start_date: DateTime<Utc>,
    timed: bool,
    to: &'a str,
}

impl<'a> Commitment<'a> {

}