//组合器
fn main() {

    //跟布尔关系的与/或很像，这两个方法会对两个表达式做逻辑组合，最终返回 Option / Result。
    //
    // or()， 表达式按照顺序求值，若任何一个表达式的结果是 Some 或 Ok，则该值会立刻返回
    // and()，若两个表达式的结果都是 Some 或 Ok，则第二个表达式中的值被返回。
    //        若任何一个的结果是 None 或 Err ，则立刻返回。
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");

    assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
    assert_eq!(n.or(n), n);    // None1 or None2 = None2

    assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
    assert_eq!(o1.or(e1), o1); // Ok or Err = Ok

    assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
    assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1

    assert_eq!(o1.and(e1), e1); // Ok and Err = Err
    assert_eq!(e1.and(o1), e1); // Err and Ok = Err


    //map() 和 map_err()
    //map 可以将 Some 或 Ok 中的值映射为另一个
    let s1 = Some("abcde");
    let s2 = Some(5);
    let trans_fn = |s: &str| s.chars().count();
    assert_eq!(s1.map(trans_fn), s2);  // Some1 map = Some2

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);
    assert_eq!(o1.map(trans_fn), o2); // Ok1 map = Ok2

    // 但是如果你想要将 Err 中的值进行改变，
    // map 就无能为力了，此时我们需要用 map_err
    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);
    let trans_fn2 = |s: &str| -> isize { s.parse().unwrap() };
    assert_eq!(e1.map_err(trans_fn2), e2); // Err1 map = Err2
}


//自定义错误
// use std::fs::File;
// use std::io::{self, Read};
// use std::num;
// #[derive(Debug)]
// struct AppError {
//     kind: String,
//     message: String,
// }
// 
// impl From<io::Error> for AppError {
//     fn from(error: io::Error) -> Self {
//         AppError {
//             kind: String::from("io"),
//             message: error.to_string(),
//         }
//     }
// }
// 
// impl From<num::ParseIntError> for AppError {
//     fn from(error: num::ParseIntError) -> Self {
//         AppError {
//             kind: String::from("parse"),
//             message: error.to_string(),
//         }
//     }
// }
// 
// fn main() -> Result<(), AppError> {
//     let mut file = File::open("./test.txt")?;
// 
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
// 
//     content = content.trim().parse().unwrap();
//     let _number: usize;
//     _number = content.parse()?;
// 
//     Ok(())
// }
