// rustdoc -p <target1> -p <target2>

//! # Restaurant
//!
//! `restaurant` is a collection of utilities to make performing certain
//! calculations more convenient.

use rand::{thread_rng, Rng};


/// this is `front_of_house` module.
pub mod front_of_house;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn eat_at_restaurant() {
    println!("I am eat_at_restaurant");
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    println!("{}", x);
    println!("{:?}", rng.gen::<(f64, bool)>());
    front_of_house::hosting::add_to_waitlist()
    
}
