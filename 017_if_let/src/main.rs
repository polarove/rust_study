fn main() {
    let v = Some(3);

    to_match(v);
    if_let(v);

}

fn to_match(v:Option<i32>){
    match v {
        Some(3) => println!("three"),
        _ => println!("not three"),
    }
}

fn if_let(v:Option<i32>){
    if let Some(3) = v {
        println!("three")
    } else{
        println!("not three")
    }
}
