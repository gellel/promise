mod action;
mod contract;
mod promise;
mod task;
mod user;

use uuid::{Uuid};
use chrono::Utc;
use promise::{Promise};
use user::{User};

fn main() {
    
    let mut user_a: User = User::new();

    let mut promise: Promise = Promise::new(user_a.id);

    let mut user_b: User = User::new();

    user_a.insert_promise(promise.id);

    promise.insert_to(user_b.id);

    
    println!("{}", user_a.to_string());

    println!("{}", promise.to_string());

}