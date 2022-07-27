pub fn test_panic_lib(num: i32) {
    if num >= 0 {
        panic!("true message")
    } else {
        panic!("false message")
    }
}