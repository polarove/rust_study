fn main() {
    println!("Hello, world!");
    another_function(23, 5);
    let x = five(3);
    println!("x = {}", x);
}

fn another_function(x: i32, h: i32) {
    println!("Another function, x = {}, h = {}", x, h);
}

fn five(xs: i32) -> i32 {
    // it will not return the result,
    // because it is not the last expression && it's a statement ending with ';'
    // 5 + xs;

    println!("caculating...");
    // it will return the result, because it is the last expression
    5 + xs
}
