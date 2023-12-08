fn main() {
    // I. what is a struct?
    // there are three types of struct:
    // 1. tuple struct
    what_is_tuple_struct();

    // 2. unit struct
    what_is_unit_struct();

    // 3. struct with two or more fields with certain types and filed name
    let user1 = what_is_a_struct();
    println!("  ");

    // II. update a struct instance
    update_user(&user1);
    println!("  ");

    // III. struct with methods
    // create a Rectangle instance
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // borrow the rect instance
    area(&rect);
    // still available here
    println!("rect: {:?}", rect);
    println!("rect: {:#?}", rect);
    println!();

    // implement functions for a struct
    println!("rect:{:?}",rect.area());
    println!("holds:{:?}",rect.can_hold(&Rectangle{width: 10, height: 40}));
    println!();

    // implement associated functions for a struct, somekind of static method in java
    let square = Rectangle::square(3);
    println!("square:{:?}",square.area());

    // multiple impl blocks
    square.print();
}

// tuple struct
struct Pair(i32, f32);

// unit struct
// to implement some trait on a type but don't have any data that you want to store in the type itself
struct Unit;

struct User {
    name: String,
    age: u8,
    username: String,
    password: String,
    is_active: bool,
}

fn what_is_tuple_struct() {
    // tuple struct is a struct without named fields
    // tuple struct is useful when you want to give the whole tuple a name and make the tuple be different from other tuples
    let tuple_struct = Pair(1, 2.0);
    println!("tuple struct: {}, {}", tuple_struct.0, tuple_struct.1);
}

fn what_is_unit_struct() {
    // unit struct is a struct without any fields
    // unit struct is useful when you want to implement a trait on some type
    // but don't have any data that you want to store in the type itself
    let _unit_struct = Unit;
}

fn what_is_a_struct() -> User {
    // 1. we must fill each field with a value
    // 2. once the struct is mutable, we can change the value of each field
    // 3. either all the fields are mutable or none of them are
    // 4. struct can be the return value of a function

    println!("create a new struct instance of user");
    let mut me = User {
        name: String::from("John"),
        age: 30,
        username: String::from("john"),
        password: String::from("123456"),
        is_active: true,
    };

    // rewrite the value of a field
    me.name = String::from("John Doe");

    println!("{}", me.name);
    println!("{}", me.age);
    println!("{}", me.username);
    println!("{}", me.password);
    println!("{}", me.is_active);
    me
}

fn update_user(previous_user: &User) -> User {
    // method 1: update the value of a field
    println!("user John will be updated to liu_qi");
    let liu_qi = User {
        name: String::from("liu_qi"),
        age: previous_user.age,
        username: String::from("liu_qi"),
        password: String::from("6602"),
        is_active: previous_user.is_active,
    };
    println!("{}", liu_qi.name);
    println!("{}", liu_qi.age);
    println!("{}", liu_qi.username);
    println!("{}", liu_qi.password);
    println!("{}", liu_qi.is_active);
    println!("  ");

    // method 2: fill the rest fields with the value of the liu_qi instance above
    println!("user liu_qi will be updated, age + 1");
    let liu_qi = User {
        name: String::from("liu_qi"),
        age: previous_user.age + 1,
        ..liu_qi
    };
    println!("liu_qi ages: {}", liu_qi.age);
    liu_qi
}

// III. struct with methods
// annotate the struct with the derive -> Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// implement functions for a struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// multiple impl blocks
impl Rectangle {
    fn print(&self) {
        println!("width: {}, height: {}", self.width, self.height);
    }
}


// IV. in the end
// we should be careful about the ownership of a struct instance
// 1. if a struct instance contains a field of String type, the struct instance will own the field => String, Vec<T>, Box<T>, etc.
// 2. if a struct instance contains a field of a primitive type, the struct instance will copy the value of the field => i32, f32, bool, char, etc.
// 3. if a struct instance contains a field of a struct type, the struct instance will own the field
// 4. if a struct instance contains a field of a reference type, the struct instance will borrow the field
