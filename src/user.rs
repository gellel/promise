use chrono::{DateTime, Utc};
use names::{Generator, Name};
use std::fmt;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug)]
pub struct User<'a> {
    pub active: HashSet<&'a str>,
    pub broken: HashSet<&'a str>,
    pub created: DateTime<Utc>,
    pub fulfilled: HashSet<&'a str>,
    pub id: Uuid,
    pub name: &'a str,
    pub relationships: HashSet<&'a usize>,
}

impl<'a> User<'a> {

    pub fn new(name: &'a str) -> User<'a> {
        User {
            active: HashSet::new(),
            broken: HashSet::new(),
            created: Utc::now(),
            fulfilled: HashSet::new(),
            id: Uuid::new_v4(),
            name: name,
            relationships: HashSet::new(),
        }
    }
}

impl fmt::Display for User<'_> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for User<'static> {

    fn default() -> User<'static> {
        User {
            active: HashSet::new(),
            broken: HashSet::new(),
            created: Utc::now(),
            fulfilled: HashSet::new(),
            id: Uuid::new_v4(),
            name: &""[..],
            relationships: HashSet::new(),
        }
    }
}