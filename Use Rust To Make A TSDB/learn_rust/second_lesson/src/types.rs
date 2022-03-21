use num::complex::Complex;

fn main() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // 对于较长的数字，可以用_进行分割
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);

    //数值上使用方法 判定未定义行为
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("undefined")
    }
    //序列(Range)
    for i in 0..10 {
        print!(" {}", i);
    }
    println!();

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
    ///
    /// Rust 拥有相当多的数值类型. 因此你需要熟悉这些类型所占用的字节数，这样就知道该类型允许的大小范围以及你选择的类型是否能表达负数
    /// 类型转换必须是显式的.      Rust 永远也不会偷偷把你的 16bit 整数转换成 32bit 整数
    /// Rust 的数值上可以使用方法.
    ///
    let c1 = 'z';
    let c2 = 'ℤ';
    let c3 = '国';
    let c4 = '😻';
    println!(" c1 用了{}字节的内存大小", std::mem::size_of_val(&c1));
    println!(" c2 用了{}字节的内存大小", std::mem::size_of_val(&c2));
    println!(" c3 用了{}字节的内存大小", std::mem::size_of_val(&c3));
    println!(" c4 用了{}字节的内存大小", std::mem::size_of_val(&c4));

    //string & slice
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    /// 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，
    /// 也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节,下面的代码就会崩溃
    let s = "中国人";
    // let a = &s[0..2];
    let a = &s[0..3];
    println!("{}", a);
    //对str的
    // let str = "hero".to_string();
    // let e = str[0];
    // println!("e: {}", e);
    //如何遍历呢？
    for c in "中国人".chars() {
        print!("{}", c);
    }
    println!();
}

fn str_example() {
    // 创建一个空String
    let mut s = String::new();
    // 将&str类型的"hello,world"添加到s中
    s.push_str("hello,world");
    // 将字符'!'推入s中
    s.push('!');
    // 最后s的内容是"hello,world!"
    assert_eq!(s, "hello,world!");

    // 从现有的&str切片创建String类型
    let mut s = "hello,world".to_string();
    // 将字符'!'推入s中
    s.push('!');
    // 最后s的内容是"hello,world!"
    assert_eq!(s, "hello,world!");

    // 从现有的&str切片创建String类型
    // String与&str都是UTF-8编码，因此支持中文
    let mut s = String::from("你好,世界");
    // 将字符'!'推入s中
    s.push('!');
    // 最后s的内容是"你好,世界!"
    assert_eq!(s, "你好,世界!");

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
}
