use modules::front_of_house;

use std::fmt;
use std::io::{self};

fn function1() -> fmt::Result {
    Ok(())
}

mod test_mod;

fn function2() -> io::Result<String> {
    Ok(String::from("hello, world!"))
}

fn main() {
    let s = function1();

    let d = function2();
    front_of_house::serve();
    test_mod::test();
    println!("s: {:?}", s);
    println!("d: {:?}", d);
}
