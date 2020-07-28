mod front_of_house {
    // Exposing Paths with the pub Keyword
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Re-exporting Names with pub use
pub use crate::front_of_house::hosting;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    crate::front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // Starting Relative Paths with super
        super::serve_order();
    }

    fn cook_order() {}

    // Making Structs and Enums Public
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

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// Providing New Names with the as Keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn new_function1() -> Result {
    // --snip--
}

fn new_function2() -> IoResult<()> {
    // --snip--
}

// Using Nested Paths to Clean Up Large use Lists
use std::{cmp::Ordering, io};

/**
use std::io;
use std::io::Write;
*/
use std::io::{self, Write};

// The Glob Operator bring all public items defined in a path into scope, we can specify that path followed by *, the glob operator
use std::collections::*;
