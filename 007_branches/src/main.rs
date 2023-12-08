fn main() {
    let num = 5;

    // expression after if must be bool, otherwise error
    if num < 5 {
        println!("condition was true");
    } else if num == 5 {
        println!("the value equals to 5");
    } else {
        println!("condition was false");
    }

    // if there are multiple conditions, use match
    // match is like switch in other languages
    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        5 => println!("five"),
        _ => println!("other"),
    }

    // ternary operators
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number)
}
