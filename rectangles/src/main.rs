
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
       self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle)-> bool {
        self.width > other.width && self.height > other.height
     }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 50,
    };
    let res = rect2.can_hold(&rect3);
    println!("the area of the rectangle is {} square pixels.", area(width1, height1));
    println!("the area of the rectangle is {} square pixels.", area2(rect1));
    println!("the area of the rectangle is {} square pixels.", area3(&rect2));
    println!("the rect is {:#?}", rect2);
    dbg!(&rect2);
    println!("eeeee{}", rect2.area());
    println!("eeeee{}", rect2.width());
    println!("result is {}", res);
}

fn area(width: u32, height: u32) ->u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

