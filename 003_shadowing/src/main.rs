fn main() {
    // initialize a variable
    // here, it should be 5
    let x = 5;
    println!("The value of x is: {}", x);

    // reassign the variable, which is shadowing
    // here, it should be 8
    let x = x + 3;
    println!("The value of x is: {}", x);

    // multiply the variable, here, it should be 16
    let x = x * 2;
    println!("The value of x is: {}", x);
}
