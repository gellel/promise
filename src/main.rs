
mod user;
use user::{User};

fn main() {

   let u = User::default();
   
   //  g = &Generator::with_naming(Name::Plain).next().unwrap()[..];

   println!("{}", u);
   // println!("{}", g);
}
