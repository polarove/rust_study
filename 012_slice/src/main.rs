fn main() {
    let d = String::from("Hello world!");

    // I. slice a string literal
    // here is a problem
    // we can't garantee the string is not changed by other code
    // so we may get a wrong result
    let unsafe_first_word = first_blank_index(&d); // 👈 immutable borrow occurs here
    let unsafe_first_word = &d[..unsafe_first_word];
    // other code:
    // d.clear(); // 👈 mutable borrow occurs here since we have cleared the string
    println!("unsafe_first_word: {}", unsafe_first_word); // 👈 immutable borrow later used here, this is not allowed in rust
    println!(" "); // thus, codes above is not safe enough in rust to get a slice from a String

    // II. what is slice
    // however, rust provides slice to solve this problem
    what_is_slice();
    println!(" ");

    // III. how to get a slice safely
    let safe_first_word = get_first_word_safely(&d);
    println!("safe_first_word: {}", safe_first_word);
    println!(" ");

    // IV. what is the type of str
    // if you have rust-analyzer installed, you can see the type of str is &str
    // which is equal to the result of get_first_word_safely(&d)
    // so, &str is a slice of a string literal
    let s = "Hello world!";
    println!("the type of s is &str: {}", s);
    println!(" ");

    // V. use slice to call a function
    println!("passing_a_str: {}", passing_a_str(&s)); // passing a string literally
    println!(" ");
    println!("passing_a_sliced_string: {}", passing_a_str(&d[..5])); // 👈 d is type of String, so we need to slice it to get a &str
    println!(" "); // 👈 so it's highly suggested to take &str as a parameter of a function, unless we want to strictly control the ownership by using String

    // VI: slice a array
    slice_array();
}

fn first_blank_index(s: &String) -> usize {
    println!("unsafe first blank index:");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // find the first blank space
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn what_is_slice() {
    println!("what_is_slice:");
    // notice that the range of slicing a String should be inside the boudary of UTF-8
    // let asd = String::from("😀😀😀😀😀😀");
    // let asd1 = &asd[..2]; // 👈 this will cause panic
    // println!("{}", asd1);

    // declare a string
    let d = String::from("Hello world!");

    // get a slice from 0 to 5
    let hello = &d[0..5];
    let hello1 = &d[..5];
    println!("{} is equal to {}", hello, hello1);
    println!(" ");

    // get a slice from 6 to 12
    let world = &d[6..12];

    // get a slice from 6 to the end
    let world1 = &d[6..];
    let world2 = &d[6..d.len()];
    println!("{} is equal to {}, {}", world, world1, world2);

    // get the whole string
    let hello_world = &d[..];
    let hello_world1 = &d[0..d.len()];
    println!("{} is equal to {}", hello_world, hello_world1);
    // this will garantee that the result is synchronized with the string whenever the string has changed or not
}

fn get_first_word_safely(s: &String) -> &str {
    println!("safely rewrite a String literal to a slice:");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // find the first blank space
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn passing_a_str(s: &str) -> &str {
    &s[..5]
}

fn slice_array() {
    println!("slice an array:");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
    println!(" ");
}
