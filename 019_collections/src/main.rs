use std::fmt::Debug;

fn main() {
    vectors();
    deal_with_index();
    ownership();
    traverse();
    store_different_data_type_in_vectors();
    pr();
}

fn vectors() {
    let mut v = Vec::new();
    v.push("value");
    v.push("asd");
    v.push("value");
    v.push("asd");
    v.push("value");
    v.push("asd");
    v.push("value");
    v.push("asd");
    v.push("value");
    v.push("asd");
    println!("{:#?}", v);

    v.pop();

    println!("{:#?}", v);
}

fn deal_with_index() {
    let v = vec![1, 23, 4, 531, 5, 36];

    // index out of boundary, which will cause a panic
    // let third = &v[100];
    // println!("the third element is {}", third);

    // handle index out of boundary
    match v.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("the third element is missing"),
    }
}

fn ownership() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let first = v.first();

    //  可能会导致内存重新分配，因此不允许不可变引用和可变引用同时存在
    // v.push(4);

    println!("The first element is {:?}", first);
}

fn traverse() {
    let mut v = vec![1, 2, 4535, 32, 2];

    for i in &mut v {
        *i += 50;
    }

    println!("{:#?}", v)
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn store_different_data_type_in_vectors() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(23.2),
        SpreadsheetCell::Text(String::from("啊实打实的阿松大")),
    ];

    println!("{:#?}", row)
}

fn pr() {
    let s = String::from("卧槽");
    println!("{}", &s);
}
