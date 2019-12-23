mod action;
mod contract;
mod promise;
mod task;
mod user;

use uuid::Uuid;
use chrono::Utc;

use user::{User};

fn main() {

    let p = promise::Promise{
        actions: None,
        broken: None,
        categories: None,
        completed: false,
        completed_at: None,
        contracts: None,
        created_at: Utc::now(),
        description: None,
        end_at: None,
        id: Uuid::new_v4(),
        kept: None,
        labels: None,
        name: None,
        start_at: None,
        subcategories: None,
        to: None,
        user_id: Uuid::new_v4(),
    };

    let mut u = User::new();

    println!("{}", u.add_promise(&p));

    for x in u.promises.unwrap().iter() {
        println!("{}", x);
    }

    println!("{}", p.id);
}