mod action;
mod contract;
mod promise;
mod user;

use action::{Action};
use contract::{Contract};
use promise::{Promise};
use user::{User};

fn main() {

    let u = User::new();

    println!("{}", u.to_string());
}
