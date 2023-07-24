use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("需要猜的数字是: {secret_number}");
    println!("请输入数字！");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // 将guess转换成number类型
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
