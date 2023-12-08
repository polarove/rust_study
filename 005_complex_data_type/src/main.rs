fn main() {
    println!("Hello, world!");

    // tuple
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, '😂');
    println!("tuple: {:?}", tup);
    println!("tuple.0: {}", tup.0);
    println!(
        "elements in tuple: {}, {}, {}, {}",
        tup.0, tup.1, tup.2, tup.3
    );

    let tup = (500, 4.32, 1, '😂');

    // destructure
    let (x, y, z, w) = tup;
    println!("destructure: {}, {}, {}, {}", x, y, z, w);

    // array
    // store data on stack
    // fixed length
    let a53: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {:?}", a53);
    let ag34: [i32; 32] = [0; 32];
    println!("array: {:?}", ag34);

    // array of months
    let months = [
        "January",   // 0
        "February",  // 1
        "March",     // 2
        "April",     // 3
        "May",       // 4
        "June",      // 5
        "July",      // 6
        "August",    // 7
        "September", // 8
        "October",   // 9
        "November",  // 10
        "December",  // 11
    ];
    // index out of bounds
    // println!("months[12]: {}", months[12])
    for month in months.iter() {
        println!("month: {}", month);
    }
    println!("months: {:?}", months);
    println!("months[0]: {}", months[0]);

    // vector
    // store data on heap
    // growable
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    for i in 3..=8 {
        v.push(i);
    }
    println!("vector: {:?}", v);
}
