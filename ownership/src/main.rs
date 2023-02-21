// fn main() {
//     // let mut s = String::from("Hello");
//     // s.push_str(", world");
//     // println!("{}", s);

//     //s进入作用域
//     let s = String::from("hello");
//     //s的值移动到函数里
//     takes_ownership(s);
//     //...所以到这里不再有效
//     // println!("{}", s);

//     let x = 5;
//     makes_copy(x);
    
// }
fn main() {
    //gives_ownership 将返回值转移给s1
    let s1 = gives_ownership();

    //s2进入作用域
    let s2 = String::from("hello");
    
    //s2被移动到takes_and_gives_back
    let s3 = takes_and_gives_back(s2);

    let s4 = String::from("hello");

    let (s2, len) = calculate_length(s4);
    
    println!("The length of '{}' is {}.", s2, len);

}
//这里 s3移出作用域被丢弃，s2也移出作用域，但已经被移走


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn gives_ownership() -> String {
    //gives_ownership会将返回值移动给调用它的函数
    //some_string 进入作用域
    let some_string = String::from("yours");
    // 返回some_string 并移出给调用的函数
    some_string
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

