extern crate test;

fn test_panic(num: i32) {
    if num >= 0 {
        panic!("true message")
    } else {
        panic!("false message")
    }
}

fn test_add(i: i32) -> i32 {
    i + 1
}


fn main() {
    assert_eq!(1,1);
    assert_ne!(1,2);
    assert!(true);
    debug_assert!(true);
}

// cfg(test) 为条件编译
#[cfg(test)]
mod tests {
    use crate::{test_add, test_panic};
    use super::*;
    use test::Bencher;

    #[test] //test 为属性标注
    fn test_case0() {
        assert_eq!(test_add(1), 2);
    }

    //失败的测试与自定义失败信息
    // #[test]
    // fn test_case1() {
    //     let left = 1 + 1;
    //     let right = 3;
    //     assert_eq!(left, right, "we expect equal but we got {} and {}", left, right)
    // }

    //关于 #[should_panic] 与 #[should_panic(expected = "true")]
    #[test]
    #[should_panic(expected = "true")]
    fn test_case2() {
        test_panic(1);
    }

    // 串行测试
    // cargo test -- --test-threads=1
    // 指定一部分测试用例
    // cargo test test_case4
    //tests -- --ignored只运行被ignored的函数
    //-- --show-output
    #[test]
    fn test_case4() {
        println!("run me only");
    }

    //dev-dep

    // #[bench]
    // fn bench_add_two(b: &mut Bencher) {
    //     b.iter(||
    //         for i in 0..10000 {
    //             test::black_box(test_add(0));
    //         });
    // }

}
