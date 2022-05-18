// from https://course.rs/basic/method.html

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//使用 impl 来定义方法
impl Rectangle {
    // new是Rectangle的关联函数
    //new 通常用于创建实例
    pub fn new(x:u32, y:u32) -> Self{
        Self{
            width:x,
            height:y,
        }
    }
    // pub fn new(x: u32, y: u32) -> Rectangle {
    //     Rectangle {
    //         width: x,
    //         height: y,
    //     }
    // }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn width(&self) -> u32 {
        return self.width;
    }
    pub fn set_width(&mut self, x: u32) {
        self.width = x;
    }
    pub fn take_ownership(self) {
        println!("self {:?}", self);
    }
}

//
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }


// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle::new(10,15);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
//      //
//      // self 表示 Rectangle 的所有权转移到该方法中
//      // &self 表示该方法对 Rectangle 的不可变借用
//      // &mut self 表示可变借用
//      // self 的使用就跟函数参数一样，要严格遵守 Rust 的所有权
//      //
//     // rect1.take_ownership();
//     println!("{}", rect1.width);
//     println!("{}", rect1.width());
//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
// }

// 枚举实现方法
fn main() {
    #[derive(Debug)]
    enum Action { //enum 像struct 去定义方法 ？ enum 编译器到底是怎么搞？
        Add,
        Subtract,
    }

    impl Action {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    let n = Action::Add;
    println!("{:?}", n.run(2,1));

    let m = Action::Subtract;
    println!("{:?}", m.run(2,1));
}
