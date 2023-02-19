//rust中的输入输出库
use rand::Rng;
use std::{cmp::Ordering, io};

//main 函数：程序入口
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is:{secret_number}");
    loop {
        println!("Please input your guess.");

        //rust中 let 创建的变量是不可变的，使用mut来使变的可变
        //let apples = 5   不可变
        //let mut apples = 5  可变

        //new 是一个静态方法 ::new 语法和C++类似
        //创建了一个可变变量，当前它绑定到一个新的String空实列上
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //rust允许改变相同变量名的类型
        //trim去除空格
        //parse 可以修改变量的类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let x = 5;
        // let y = 10;
        // println!("x = {x}, and y + 2 = {}", y + 2);
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
