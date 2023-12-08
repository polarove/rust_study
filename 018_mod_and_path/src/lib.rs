// everything is immutable in implicit unless you specify it as mut
// everything is private in implicit unless you specify it as pub

// 1. what is absolute path?
// crate-level, or file-system-level
// absolute path starts with crate name or keyword self
// absolute path is used to call the function in the same crate

// 2. what is relative path?
// file-level
// relative path starts with mod name
// relative path is used to call the function in the same file

// 3. what is super?
// mod-level
// super is used to call the function in the parent mod

mod new_mod;

fn serve_order(types: &str) {
    println!("serve_order through: {}", types);
}

pub mod front_of_house {
    pub fn serve() {
        super::serve_order("super::");
    }
    pub fn take_payment() {
        println!("take_payment");
    }
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
            super::super::serve_order("super::super");
        }
        pub fn seat_at_table() {
            println!("seat_at_table");
            crate::serve_order("crate::");
            super::take_payment();
        }
    }
}

// private
pub mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        // each field should be clear with access control modifier
        pub calorie: i32,
        name: String,
    }

    // each field in enum follows the top-level defination of access control
    pub enum Time {
        BREAKFAST,
        LAUNCH,
        DINNER,
    }

    impl Breakfast {
        pub fn baken(name: String) -> Breakfast {
            Breakfast { calorie: 23, name }
        }
    }
}

pub fn start() {
    // Absolute path
    // since both front_of_house and eat_at_restaurant are in the same crate
    // mod front_of_house can be called directly through crate:: or self::

    // since both front_of_house and eat_at_restaurant are in the same file
    // mod front_of_house can be called directly without pub keyword specified
    crate::front_of_house::hosting::add_to_waitlist();
    self::front_of_house::hosting::add_to_waitlist();

    crate::front_of_house::serve();

    // Relative path
    front_of_house::hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::baken(String::from("Peaches"));
    meal.calorie = 4242;
    println!("meal: {:#?}", meal);

    // this will cause an error since field name is private
    // meal.name = String::from("value");

    new_mod::say();
}

// the use key word
// just like import { xxx } from '@/...' in es6
// what's been used will be accessible in current file
// and it follows the access control rules
// which means we can't call the private functions inside the mod

// by absolute path
use crate::front_of_house::hosting;

// by relative path
// use front_of_house::hosting;

pub fn eat_at_home() {
    hosting::add_to_waitlist();
}
