mod user;
use user::{User};

fn main() {
    println!("Hello, world!");

    let u = User::new(String::from("Rust"));

    println!("{}", u.id);
}
