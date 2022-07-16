
/*
crate
 └── test_func
 └── testA
     ├── testA_mod
     │   ├── func1
     │   └── func2
     └── testB_mod
         ├── func3
         ├── func4
         └── func5
*/

mod test;
pub mod pub_use;




mod testA {
    pub(crate) mod testA_mod {
        pub(crate) fn func1() {}

        fn func2() {}
    }

    mod testB_mod {
        fn func3() {}

        fn func4() {}

        fn func5() {}
    }
}

/// 代码可见性
/// Rust 出于安全的考虑，默认情况下，所有的类型都是私有化的，包括函数、方法、
/// 结构体、枚举、常量，是的，就连模块本身也是私有化的
///
/// 将结构体设置为 pub，但它的所有字段依然是私有的
/// 将枚举设置为 pub，它的所有字段也将对外可见


/// 调用一个函数，就需要知道它的路径，在 Rust 中，这种路径有两种形式：
///
/// 绝对路径，从包根开始，路径名以包名或者 crate 作为开头
/// 相对路径，从当前模块开始，以 self，super 或当前模块的标识符作为开头
///
/// 使用 self 引用模块
/// self 其实就是引用自身模块中项，
pub fn test_func() {

    // 用路径引用模块
    // 绝对路径
    crate::testA::testA_mod::func1();
    // 相对路径
    self::testA::testA_mod::func1();
    testA::testA_mod::func1();
}



/// 更好的方式 使用use 引入 模块或者函数
//use crate::testA::testA_mod::func1;
//use crate::testA::testA_mod;

use crate::testA::testA_mod::*;
pub fn test_func1() {
    // 用路径引用模块
    func1();
    //testA_mod::func1();
}


/// 引入模块还是函数
/// 尽量保证最小化的引入规则
/// 使用 * 表示全部引入 暴力 一般用在写测试用例的场景下 简单直接


/// 使用 super 引用模块
/// super 代表的是父模块为开始的引用方式，
/// 非常类似于文件系统中的 .. 语法：../a/b 文件名：
fn funA() {}
mod testB {
    fn test_func2() {
        func();
        super::funA();
    }

    fn func() {}
}