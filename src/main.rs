mod step;
mod user;

use step::{Step};
use user::{User};

fn main() {

   let mut u = User::default();

   let s = Step::new(String::from("do something"));

   u.name = String::from("hello");
   
   println!("{}", u);

   println!("{:?}", s);

}
