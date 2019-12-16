use chrono::{DateTime, Utc};
use std::collections::{HashSet};
use std::{fmt};
use uuid::{Uuid};

#[derive(Debug)]
pub struct User<'a> {
    pub active: HashSet<&'a str>,
    pub broken: HashSet<&'a str>,
    pub created: DateTime<Utc>,
    pub fulfilled: HashSet<&'a str>,
    pub id: Uuid,
    pub name: String,
    pub relationships: HashSet<&'a usize>,
}

impl<'a> User<'a> {

    pub fn new(name: String) -> User<'a> {
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
            name: Uuid::new_v4().to_hyphenated().to_string(),
            relationships: HashSet::new(),
        }
    }
}