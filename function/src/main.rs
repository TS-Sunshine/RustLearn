fn main() {
    println!("Hello, world!");
    another_function(5, 'h');

    let x = five();
    print!("Main the value of x is {x}");
}


fn five() -> i32 {
    return 10;
}

fn another_function(x: i32, unit_char: char) {
    println!("Another function");
    println!("The value of x is: {x}{unit_char}");

    //表达式和语句
    //在rust中语句不返回值 let y = 6;是语句， 不返回值，所以不能将值绑定到x上
    // let x = (let y = 6);
    let x = {
        let y = 10;
        y + 1
    };

    println!("The value of x is {x}");
}