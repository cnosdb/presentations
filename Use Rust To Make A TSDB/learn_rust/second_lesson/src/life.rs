// 就像编译器大部分时候可以自动推导类型 <-> 一样，编译器大多数时候也可以自动推导生命周期
// 在多种类型存在时，编译器往往要求我们手动标明类型 <-> 当多个生命周期存在，
// 且编译器无法推导出某个引用的生命周期时，就需要我们手动标明生命周期

//借用检查器(Borrow checker)
//保证 Rust 的所有权和借用的正确性，Rust 使用了一个借用检查器(Borrow checker)
// fn main() {
//     {
//         let r;
//         {
//             let x = 5;
//             r = &x;
//         }
//         // println!("r: {}", r);
//     }
// }

//
// 函数中的生命周期
// 生命周期标注并不会改变任何引用的实际作用域
//
// 函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：
// 1. 函数参数的生命周期
// 2. 函数体中某个新建引用的生命周期
// fn main() {
//     let string2 = "cnosdb";
//     let string1 = String::from("timeseries database");
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn main() {
//     let string2 = "cnosdb";
//     let result;
//     {
//         let string1 = String::from("timeseries database");
//         result = longest(string1.as_str(), string2);
//     }
//     println!("The longest string is {}", result);
// }
//x、y 和返回值至少活得和 'a 一样久(因为返回值要么是 x，要么是 y)
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 结构体中的生命周期
#[derive(Debug)]
struct Test<'a> {
    temp: &'a str,
}

fn main() {
    let i;
    {
        let hi = "hello world";
        let word = hi.split(' ').next().unwrap();
        i = Test { temp: word };
    }
    println!("{:?}", i);

    // 静态生命周期
    // 在 Rust 中有一个非常特殊的生命周期，那就是 'static，拥有该生命周期的引用可以和整个程序活得一样久。
    // 在之前我们学过字符串字面量，提到过它是被硬编码进 Rust 的二进制文件中，因此这些字符串变量全部具有 'static 的生命周期：
    let l: &'static str = "hello world!";
}

// 生命周期的消除规则
// 1.每一个引用参数都会获得独自的生命周期
// 2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋
//   给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期
// 3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
//   拥有 &self 形式的参数，说明该函数是一个 方法，该规则让方法的使用便利度大幅提升。
//   self 是什么下次分享为你揭晓？
//方法中的生命周期
//
