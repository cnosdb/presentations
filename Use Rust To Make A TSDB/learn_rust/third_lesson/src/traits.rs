use std::rc::Rc;

/// trait定义了一个可以被共享的行为，只要实现了trait，就能使用该行为
pub trait Node {
    fn move_to(&mut self, x: f32, y: f32);
    fn draw(&self);
}

struct EmptyNode {
    x: f32,
    y: f32,
}

impl Node for EmptyNode {
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self) {
        println!("EmptyNode: x={}, y={}", self.x, self.y)
    }
}

struct TestNode {
    x: f32,
    y: f32,
}
impl Node for TestNode {
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self) {
        println!("TestNode: x={}, y={}", self.x, self.y)
    }
}

impl TestNode {
    fn test(&self){
        println!("test");
    }
}

/// 特征对象 dyn
/// 使用特征对象来代表泛型或具体的类型
fn draw1(x: Box<dyn Node>) {
    x.draw();
}

fn draw2(x: &dyn Node) {
    x.draw();
}

// fn main() {
//     let test = EmptyNode{x: 1_f32, y:2_f32};
//     test.draw();
//
//     let test2 =TestNode{x:1.2, y:1.3};
//
//     draw2(&test);
//     draw2(&test2);
//
//     draw1(Box::new(test));
//     draw1(Box::new(test2));
// }


// 总结一下：
// draw1 函数的参数是 Box<dyn Draw> 形式的特征对象，该特征对象是通过 Box::new(x) 的方式创建的
// draw2 函数的参数是 &dyn Draw 形式的特征对象，该特征对象是通过 &x 的方式创建的
// dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn
//  dyn 本质：
//  1. 特征对象大小不固定 但它的引用类型的大小是固定的，它由两个指针组成（ptr 和 vptr），因此占用两个指针大小
//  2. 一个指针 ptr 指向实现了特征 Node的实例，比如 EmptyNode, TestNode的实例
//     另一个指针 vptr 指向一个虚表 vtable，vtable 中保存了类型 EmptyNode/TestNode
//     的实例对于可以调用的实现于特征Node的方法。当调用方法时直接从 vtable 中找到方法并调用
//https://doc.rust-lang.org/reference/items/traits.html#object-safety%3E


// 关联类型是在特征定义的语句块中，申明一个自定义类型，这样就可以在特征的方法签名中使用该类型
// Self 用来指代当前调用者的具体类型，
// Self::Item 就用来指代该类型实现中定义的 Item 类型
struct Counter{
    num: u32,
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.num += 1;
        Some(self.num)
    }
}


// fn main() {
//     let mut c = Counter{num:1};
//     while c.num < 10 {
//         println!("num: {}", c.next().unwrap());
//     }
//     // println!("num: {}", c.count())
// }


use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// fn main() {
//     assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//                Point { x: 3, y: 3 });
// }


// 调用同名方法
trait Pilot {
    fn fly(&self);
    fn work();
}

trait Wizard {
    fn fly(&self);
    fn work();
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot");
    }

    fn work(){
        println!("Pilot working");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard");
    }

    fn work(){
        println!("Wizard working");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human");
    }

    fn work(){
        println!("Human working");
    }
}

fn main() {
    let person = Human;
    //优先调用类型上的方法
    person.fly();         // 调用Human类型自身的方法
    //显式调用的语法
    Pilot::fly(&person); // 调用Pilot特征上的方法
    Wizard::fly(&person); // 调用Wizard特征上的方法

    <Human as Pilot>::fly(&person);

    //完全限定语法
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    Human::work();
    <Human as Pilot>::work();

}


// 在外部类型上实现外部特征(newtype)
// 就是特征或者类型必需至少有一个是本地的，才能在此类型上定义特征。
// 这里提供一个办法来绕过孤儿规则，那就是使用newtype 模式，
// 简而言之：就是为一个元组结构体创建新类型。该元组结构体封装有一个字段，该字段就是希望实现特征的具体类型。
// 该封装类型是本地的 这样我们就可以为类型实现外部的特征。
// newtype 在运行时没有任何性能损耗，因为在编译期，该类型会被自动忽略。
use std::fmt;

struct Wrapper(Vec<String>);

// impl<T> fmt::Display for Vec<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.join(", "))
//     }
// }

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// fn main() {
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }
