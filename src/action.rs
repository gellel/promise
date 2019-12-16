use chrono::{DateTime, Utc};
use std::fmt;
use std::collections::HashSet;
use uuid::{Uuid};

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
    start_date: Option<DateTime<Utc>>
}