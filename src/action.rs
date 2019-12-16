mod step;

use chrono::{DateTime, Utc};
use std::fmt;
use std::collections::HashSet;
use step::{Step};
use uuid::{Uuid};

#[derive(Debug)]
pub struct Action<'a> {
    category: &'a mut str,
    completed: bool,
    created: DateTime<Utc>,
    description: &'a mut str,
    end_date: Option<DateTime<Utc>>,
    expired: bool,
    id: Uuid,
    name: &'a mut str,
    next: &'a Action,
    start_date: Option<DateTime<Utc>>,
    steps: Step,
    timed: bool,
}

impl fmt::Display for Action<'_> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
