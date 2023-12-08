use std::io;

const MIN_POINTS: u32 = 100_0000;

fn main() {
    println!("Hello, world!");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please type a number!"),
    };

    let foo = 32;
    println!("foo: {}",foo);
    // not permitted, foo is immutable
    // foo = number; 

    let foo = number; // permitted, foo is mutable

    println!("MIN_POINT: {}", MIN_POINTS);

    println!("You guessed: {}", number);

    println!("foo is: {}", foo);

    // Declaring a constant
    const MAX_POINTS: u32 = 100_000;

    // A constant variable is not allowed to be mutated
    // MAX_POINTS = 100_001; 

    println!("max points is: {}", MAX_POINTS);
}
