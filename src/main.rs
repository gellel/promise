mod cx;

use cx::{Cx};
use uuid::{Uuid};

fn main() {
    
    let mut cx = Cx::new(Uuid::new_v4(), Uuid::new_v4());

    println!("{}", cx.sign_as_from(String::from("")));

    println!("{:?}", cx);

    println!("{}", cx.sign_as_to(String::from("")));

    println!("{:?}", cx);
}