use chrono::{DateTime, Utc};
use std::fmt;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug)]
pub struct User<'a> {
    pub active: HashSet<&'a str>,
    pub broken: HashSet<&'a str>,
    pub created: DateTime<Utc>,
    pub fulfilled: HashSet<&'a str>,
    pub id: &'a str,
    pub name: &'a str,
    pub relationships: HashSet<&'a usize>,
}

impl<'a> User<'a> {

    pub fn new(id: &'a str, name: &'a str) -> User<'a> {
        User {
            active: HashSet::new(),
            broken: HashSet::new(),
            created: Utc::now(),
            fulfilled: HashSet::new(),
            id: id,
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
            id: "",
            name: "",
            relationships: HashSet::new(),
        }
    }
}