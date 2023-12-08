fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // specify the type of absent_number in Option<T> to be i32
    // so the compiler can infer that the type of absent_number is Option<i32>
    // even if the value is None
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    println!("is None: {:?}", absent_number.is_none());
}


// defination in std lib
// enum Option<T> {
//     Some(T),
//     None,
// }