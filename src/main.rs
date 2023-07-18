use std::io;

fn main() {
    println!("猜数字");

    println!("请输入数字！");

    // let mut foo = 1;
    // let bar = foo;

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数字是{}", guess);
}
