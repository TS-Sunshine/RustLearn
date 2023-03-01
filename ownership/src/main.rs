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
// fn main() {
//     //gives_ownership 将返回值转移给s1
//     let s1 = gives_ownership();

//     //s2进入作用域
//     let s2 = String::from("hello");
    
//     //s2被移动到takes_and_gives_back
//     let s3 = takes_and_gives_back(s2);

//     let s4 = String::from("hello");

//     let (s2, len) = calculate_length(s4);
    
//     println!("The length of '{}' is {}.", s2, len);

// }
//这里 s3移出作用域被丢弃，s2也移出作用域，但已经被移走





fn main() {
   let s1 = String::from("hello");

   let length = calculate_length_ref(&s1);
   print!("The length of {} is {}", s1, length);

    //错误
    // change(&s1);
    //可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("the new string is{}", s2);
    // 只能创建一个可变引用
    let mut s = String::from("mut String");
    let s1 = &mut s;
    //这时候使用创建一个对s的可变引用，报错
    // let s2 = &mut s;
    // println!("{}，{}", s1, s2);

    //不可变字符串
    let new_wrod = String::from("hello world! sanhu");
    let size = first_word(&new_wrod);
    //clear 是将字符串变为""
    // new_wrod.clear();
    println!("size = {}", size);
    let mut mut_new_word = String::from("hello world!");
    let size2 = first_word(&mut_new_word);
    mut_new_word.clear();
    println!("mut size = {}, str = {}", size2, mut_new_word);
    
    //string slice
    let mut s_slice = String::from("hello world");
    let slice_str = first_world_slice(&s_slice);
    // s_slice.clear();
    println!("slice str len = {}", slice_str);

    let ss = "ddd";
    ss = "222";
   
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
   some_string.push_str("string")
}

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


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("字节数组为:{:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
           return i;
       } 
    }
    return s.len();
}

fn first_world_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

