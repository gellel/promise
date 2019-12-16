use std::collections::{LinkedList};
use uuid::{Uuid};

#[derive(Debug)]
pub struct Step {
    id: Uuid,
    next: Option<LinkedList<Step>>,
}

impl Step {
    pub fn new() -> Step {
        Step{
            id: Uuid::new_v4(),
            next: None,
        }
    }
}