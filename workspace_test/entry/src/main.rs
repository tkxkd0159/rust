#![allow(unused)]


use restaurant::{self, front_of_house::hosting};

fn main() {
    let x = chrono::Utc::now();
    println!("{}", x);

    hosting::add_to_waitlist();
    restaurant::eat_at_restaurant();

}
