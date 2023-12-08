fn main() {
    println!("Hello, world!");

    // constant value
    const CONSTANT: &str = "CONSTANT_VALUE";
    // can't be mutated
    // CONSTANT = "NEW_VALUE";

    // u -> unsigned
    // the default type of unsigned integer in rust is u32
    let x: u32 = 24;

    // u8 -> 8 bit unsigned integer
    // ranged from 0 to 2^n-1(255)
    const MIN_U8: u8 = 0;
    const MAX_U8: u8 = 255;
    println!("MIN_U8: {}, MAX_U8: {}", MIN_U8, MAX_U8);

    // i -> integer -> either signed or unsigned
    // ranged from -2^(n-1) to 2^(n-1)-1
    // the default type of integer in rust is i32
    const MIN_I32: i32 = -2147483648;
    const MAX_I32: i32 = 2147483647;

    // i64 -> 64 bit integer, it's something like Long in Java
    const MIN_LONG: i64 = -9223372036854775808;
    const MAX_LONG: i64 = 9223372036854775807;
    println!(
        "A constant: {}, unsigned value: {}, MIN_I32: {}, MAX_I32: {}, MIN_I64: {}, MAX_I64: {}",
        CONSTANT, x, MIN_I32, MAX_I32, MIN_LONG, MAX_LONG
    );

    // f -> float
    // the default type of float in rust is f64
    // which is double precision float number
    let y: f64 = 3.142141935286093210589832;
    println!("A float value: {}", y);

    // auto type inference
    let z = 3.141592653589793238462643383279502884197169399375105820974944592307816406286;
    let we = 123 + 3;
    let we2 = 123.0 + 3.0;
    let we3 = "214";
    let we4 = false;
    let we5 = -421;
    let we6 = 123_i32;
    println!(
        "z: {}, we: {}, we2: {}, we3: {}, we4: {}, we5: {}, we6: {}",
        z, we, we2, we3, we4, we5, we6
    );

    // char in rust is 4 bytes, following the unicode standard.
    // use single quote to declare a char
    let c = 'c';
    let g: char = '✿';
    let vv: char = '꧂';
    let o: char = '😻';
    println!("c: {}, g: {}, vv: {}, o: {}", c, g, vv, o);
}
