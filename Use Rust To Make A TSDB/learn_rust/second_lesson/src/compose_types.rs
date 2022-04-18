use std::option::Option;

fn main() {
    /// 元组  tupe
    /// 元组是由多种类型组合到一起形成的，因此它是复合类型，
    /// 元组的长度是固定的，元组中元素的顺序也是固定的。
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    /// 结构体
    /// 一个结构体有几部分组成：
    /// 通过关键字 struct 定义
    /// 一个清晰明确的结构体 名称
    /// 几个有名字的结构体 字段
    ///
    #[derive(Debug)]
    struct Stu {
        weight: u32,
        age: u32,
    }

    let l = Stu {
        weight: 64,
        age: 18,
    };
    println!("struct: l: {:?}", l);

    /// 元组结构体(Tuple Struct)
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /// 单元结构体(Unit-like Struct)
    ///如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    /// 我们不关心 AlwaysEqual 的字段数据，
    /// 只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    trait SomeTrait {}
    impl SomeTrait for AlwaysEqual {}

    ///枚举类型是一个类型，
    /// 它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    enum Message {
        get(String, String),
        put(String),
    }

    ///Option 枚举用于处理空值
    /// Option 定义
    ///    enum Option<T> {
    ///         Some(T),
    ///         None,
    ///     }
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    /// array
    ///
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];
}
