

/// as 别名引用
/// 对于同名冲突问题，还可以使用 as 关键字来解决
///
use std::fmt::Result;
// use std::io::Result;
use std::io::Result as IoResult;
fn func1() -> Result {
    // --snip--
    Ok(())
}

// fn func3() -> std::io::Result<()> {
//     // --snip--
//     Ok(())
// }

fn func2() -> IoResult<()> {
    // --snip--
    Ok(())
}
