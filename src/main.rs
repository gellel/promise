mod promise;

use chrono::{Utc};
use std::collections::HashSet;
use uuid::{Uuid};

use promise::{Promise};

fn main() {

    let user_id = Uuid::new_v4().to_string();

    let mut to: HashSet<String> = HashSet::new();

    to.insert(user_id.clone());

    let promise = Promise{
        actions: None,
        category: String::from("task"),
        complete: false,
        created_at: Utc::now(),
        description: String::from("lorem ipsum"),
        end_at: None,
        id: Uuid::new_v4().to_string(),
        labels: None,
        name: String::from("walk the dog"),
        start_at: Some(Utc::now()),
        subcategory: None,
        to: to,
        user_id: user_id,
    };

    println!("{:?}", promise);

    println!("serialized = {}", promise.to_string());

}
