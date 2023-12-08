#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind2 {
    fn call(&self) {
        println!("call: {:?}", self);
    }
}

fn main() {
    what_is_enum();
    println!();
    enum_in_struct();
    println!();
    variant_in_enum();
    println!();
    enum_with_method();
}

fn what_is_enum (){
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    route(v4);
    route(v6);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_category: IpAddrKind){
    println!("ip_category: {:?}", ip_category)
}

fn enum_in_struct (){
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("124.32.42.24"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home: {:?}, kind:{:#?}, ip -> {}", home, home.kind, home.address);
    println!("loopback: {:?}, kind:{:#?}, ip -> {}", loopback, loopback.kind, loopback.address);
}

fn variant_in_enum (){
    let home = IpAddrKind2::V4(127, 0, 0, 1);
    let loopback = IpAddrKind2::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}

fn enum_with_method (){
    let home = IpAddrKind2::V4(127, 0, 0, 1);
    let loopback = IpAddrKind2::V6(String::from("::1"));
    home.call();
    loopback.call();
}