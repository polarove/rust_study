fn main() {
    loop_function();
    while_function();
    for_function();
}

fn loop_function() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter % 2 == 0 {
            println!("even");
            continue;
        }
        if counter > 50 {
            println!("exit");
            break counter + 2;
        }
    };

    println!("result: {}", result);
}

fn while_function() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_function() {
    // iterate a collection
    let a = [-10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {}", element);
    }

    // iterate over a range
    // .rev() reverses the range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    print!("LIFTOFF!!!")
}
