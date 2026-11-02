/* pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
   }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
   }
} */

fn deliver_order() {
mod back_of_house {
    fn fix_incorrect_order();
    super::deliver_order(); // using this easier if the codes stay in the same crate
    }
    fn cook_order() {}
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    //order in summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    //bread
    meal.toast = String::from("wheat");
    println!("i like {}", meal.toast);

}

mod front_of_house;

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlists
}

use std::fmt::Result;
use std::io:: Result as IoResult;
fn function1() -> fmt::Result {

}
fn function2() -> IoResult<()> {

}
use std::{cmp::Ordering, io};
use std::io::{self, Write};
