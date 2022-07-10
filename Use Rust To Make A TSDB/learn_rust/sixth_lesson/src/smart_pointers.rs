
#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {

    // let box_point = Box::new(Point { x: 0.0, y: 0.0 });
    // //实现 Deref 后的智能指针结构体，就可以像普通引用一样，通过 * 进行解引用
    // //*(box_point.deref()) -> *(&T)
    // let unboxed_point: Point = *box_point;
    // assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
    //
    // //1. 避免栈上数据的拷贝
    // let arr = [0;10];
    // let arr1 = arr;
    // // arr 和 arr1 都拥有个自数组的所有权，因此不会报错
    // println!("{:?}", arr.len());
    // println!("{:?}", arr1.len());
    //
    // let arr = Box::new([0;10]);
    // // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // // 所有权顺利转移给 arr1，arr 不再拥有所有权
    // let arr1 = arr;
    // println!("{:?}", arr1.len());
    // println!("{:?}", arr.len());

    //2. 特征对象 在面向对象中分享过 特征对象

    //3.
    //Box::leak
    //Box::leak，它可以消费掉 Box 并且强制目标值从内存中泄漏
    //改变了变量的生命周期
    println!("{}", gen_static_str());


    //dref trait
    // dref_fn();
    //drop trait
    drop_fn();
}


fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}


// Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
// Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作
fn dref_fn() {
    let x = Box::new(1);
    let sum = *x + 1;
}


struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}


//互斥的 Copy 和 Drop
// #[derive(Copy)]
#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}
fn drop_fn(){
    println!("drop_fn start!");

    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let foo = Foo;


    //foo.drop();
    drop(foo);
    println!("{:?}", foo);

    println!("drop_fn end!");
}