
fn main() {

    // pattern match
    println!("{}",value_in_color(Color::RED));
    println!("{}",value_in_color(Color::GREEN));
    println!("{}",value_in_color(Color::YELLOW));
    println!();
    // binding values in enum
    println!("{}",value_in_color(Color::BLUE(Category::LIGHT)));
    println!("{}",value_in_color(Color::BLACK(Category::DARK)));
}


enum Color{
    RED,
    BLUE(Category),
    GREEN,
    YELLOW,
    BLACK(Category),
}

fn value_in_color (color: Color) -> String {
    match color {
        Color::RED => String::from("RED"),
        Color::BLUE (category) => {
            println!("category is: {:?}",category);
            String::from("BLUE")
        },
        Color::GREEN => String::from("GREEN"),
        Color::YELLOW => String::from("YELLOW"),
        Color::BLACK(category) => {
            println!("category is: {:?}",category);
            String::from("BLACK")
        },
    }
}
#[derive(Debug)]
enum Category {
    LIGHT,
    DARK,
}
