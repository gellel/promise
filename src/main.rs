
mod user;
use user::{User};

fn main() {

   let mut u = User::default();

   u.name = String::from("hello");
   
   println!("{}", u);
}
