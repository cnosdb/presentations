
// 函数式编程有很多特点

// 函数是一等公民
// 使用函数作为参数进行传递
// 使用函数作为函数返回值
// 将函数赋值给变量


//闭包

// 闭包是一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，
// 不同于函数的是，它允许捕获调用者作用域中的值
// fn main() {
//     let x = 1;
//     // let sum = |y| x + y;
//     let sum = |y:u8| -> u8 {
//         x + y
//     };
//
//     assert_eq!(3, sum(2));
// }


// 闭包去捕获作用域中的值 而 函数不可以
fn main() {
    let x = 4;
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // 闭包作为函数返回值
    fn factory() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);
}