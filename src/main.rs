mod contract;
mod promise;

use chrono::{Utc};
use contract::{Contract};
use std::collections::HashSet;
use uuid::{Uuid};

use promise::{Promise};

fn main() {

    let user_id = Uuid::new_v4().to_string();

    let user_id_2 = Uuid::new_v4().to_string();

    let c = Contract{
        created_at: Utc::now(),
        id: Uuid::new_v4().to_string(),
        signed: false,
        signed_at: None,
        to: user_id_2.clone(),
        user_id: user_id.clone(),
    };

    let mut contracts: HashSet<String> = HashSet::new();

    contracts.insert(c.id.clone());


    let mut to: HashSet<String> = HashSet::new();

    to.insert(user_id_2.clone());

    let promise = Promise{
        actions: None,
        category: String::from("task"),
        complete: false,
        contracts: Some(contracts),
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


    println!("{}", promise.to_string());

    println!("{}", c.to_string());

}
