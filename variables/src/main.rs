

fn main() {
     //不能对不可变变量x 进行二次赋值
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

   //可变变量
//    let mut x = 5;
//    println!("The value of x is: {x}");
//    x = 6;
//    println!("The value of x is: {x}");


   //常量
   // 常量是用const 修饰的，不可变变量是let修饰 
   const  THREE_HOUSE_IN_SECONDS: i32 = 60 * 60 * 3;

   //
   //首先创建了一个变量x，绑定到5上
   let x = 5;
   //创建了一个新变量，获取之前x的值 5 + 1 赋值给x
   let x = x + 1;
   
   {
        //
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
   }

   println!("The value of x is: {x}");


   let spaces = "     ";
   let spaces = spaces.len();
   println!("spaces length is {spaces}");

//    let mut spaces = "     ";
//    spaces = spaces.len();
   
}