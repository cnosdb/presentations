
/// 如果仅仅支持通过转移所有权的方式获取一个值，那会让程序变得复杂。
/// Rust 能否像其它编程语言一样，使用某个变量的指针或者引用呢？
/// Rust 通过 借用(Borrowing) 这个概念来达成上述的目的，获取变量的引用，称之为借用(borrowing)

/// 常规引用是一个指针类型，指向了对象存储的内存地址
///
/// 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
/// 引用必须总是有效的
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题
    //
    // println!("{}, {}, and {}", r1, r2, r3);

    /// NLL
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);

    //悬垂引用(Dangling References)
    // let reference_to_nothing = dangle();
}
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }