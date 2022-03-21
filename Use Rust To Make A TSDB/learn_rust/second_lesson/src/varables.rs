fn main() {
    /// 变量
    // let x = 1;
    let mut x = 3;
    println!("x: {}", x);
    x = 4;
    println!("x: {}", x);
    {
        let x = 1;
        println!("x: {}", x);
    }

    // const y: i32 = 11;
    const Y: i32 = 11;
    println!("Y: {}", Y);

    let (y, z) = (2, 3);
    println!("y: {}, z: {}", y, z);

    ///垃圾回收机制(GC)，在程序运行时不断寻找不再使用的内存，典型代表：Java、Go
    /// 手动管理内存的分配和释放, 在程序中，通过函数调用的方式来申请和释放内存，典型代表：C++
    /// 通过所有权来管理内存，编译器在编译时会根据一系列规则进行检查

    /// Rust 中每一个值都 有且只有 一个所有者(变量)
    /// 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

    /// 转移所有权
    let s1 = String::from("hello");
    // let s2 = s1;
    println!("{}, world!", s1);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 所有的基本类型都实现了 copy trait
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
}
