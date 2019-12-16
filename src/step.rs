use std::collections::{HashSet, LinkedList};
use std::fmt;
use uuid::{Uuid};

#[derive(Debug)]
pub struct Step<'a> {
    id: Uuid,
    labels: HashSet<&'a str>,
    name: String,
    next: Option<LinkedList<Step<'a>>>,
}

impl<'a> Step<'a> {
    pub fn new(name: String) -> Step<'a> {
        Step{
            id: Uuid::new_v4(),
            labels: HashSet::new(),
            name: name,
            next: None,
        }
    }
}

impl fmt::Display for Step<'_> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}