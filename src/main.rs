mod user;
use user::{User};

fn main() {

   let u = User::new("223232", "hello");

   println!("{}", u);
}
