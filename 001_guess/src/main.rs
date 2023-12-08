use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数!！");

    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取行失败");

        if guess.trim() == "quit" {
            println!("正在退出...");
            break;
        } else if guess.trim() == "" {
            println!("请输入数字");
            continue;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
        };

        println!("你猜的数是：{}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}
