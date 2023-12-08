fn main() {
    // I. what is reference?
    what_is_reference();

    // II. what is borrowing?
    let s1 = String::from("412sdfa");
    // pass a reference to a function instead of the actual value
    what_is_borrowing(&s1);
    // s1 is still valid here, because we only passed a reference to the function
    println!("s1 is valid still here: {}", s1);

    // III. what is mutable reference?
    let mut s2 = String::from("hello");
    // pass a mutable reference to a function
    what_is_mutable_borrowing(&mut s2);
    println!("s2 is valid still here: {}", s2);

    // IV. a reference can only be borrowed once in a scope
    reference_can_only_be_borrowed_once();

    // V. we can't have a mutable reference while we have an immutable one
    exclusive_borrowing();

    // VI. dangling reference
    let reference_to_something = dangling_reference();
    println!("{}", reference_to_something);
}

fn what_is_reference() {
    // 1. reference is a pointer to a value, started with `&`
    // 2. reference is immutable by default
    // 3. reference can be mutable by using `&mut`
    // 4. reference can be dereferenced by using `*`
    // 5. reference can be destructured by using `ref` and `ref mut`
    let s1 = String::from("hello");
    let s2: &String = &s1;
    println!("s1: {}, s2: {}", s1, *s2);
}

fn what_is_borrowing(param: &String) {
    // borrowed reference is immutable by default
    let s2 = param;
    println!("{}", s2)
}

fn what_is_mutable_borrowing(param: &mut String) {
    // borrowed from main function
    param.push_str(", world");
    println!("{}", param)
}

fn reference_can_only_be_borrowed_once() {
    // what's the advantage of this rule?
    // avoids data competition at runtime

    let mut s3 = String::from("hello");
    let _r1 = &mut s3;
    // let _r2 = &mut s3; // 👈error: cannot borrow `s3` as mutable more than once at a time

    // what if we want to borrow a mutable reference twice?
    // use a new scope, but it's not suggested
    {
        let _r2 = &mut s3;
    }
}

fn exclusive_borrowing() {
    let mut s4 = String::from("hello");
    println!("{}", s4);
    
    s4 = String::from("214124");
    let r1 =  &s4;
    // 👈error: cannot borrow `s4` as mutable because it has been borrowed to r1
    // let r2 = &mut s4; 
    println!("{} and {}", r1, s4)
}

fn dangling_reference() -> String {
    // what is dangling reference?
    // a reference to a value that has been freed
    // rust compiler will prevent this from happening

    // s have exceeded its lifetime
    let s = String::from("hello");
    s
}
