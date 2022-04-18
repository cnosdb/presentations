fn main() {
    let s1 = String::from("hello");
    // let s2 = s1;
    println!("{}, world!", s1);

    // gives_ownership 将返回值移给 s1
    let s1 = gives_ownership();
    // 所有权转给了 takes_ownership 函数, s1 不可用了
    takes_ownership(s1);
    // 如果编译下面的代码，会出现s1不可用的错误
    // println!("s1= {}", s1);
    let s2 = String::from("hello"); // 声明s2
                                    // s2 被移动到 takes_and_gives_back 中, 它也将返回值移给 s3。
                                    // 而 s2 则不可用了。
    let s3 = takes_and_gives_back(s2);
    //如果编译下面的代码，会出现可不可用的错误
    println!("s2={}, s3={}", s2, s3);
    println!("s3={}", s3);
}

// takes_ownership 取得调用函数传入参数的所有权，
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 drop方法。占用的内存被释放

// gives_ownership 将返回值移动给调用它的函数
fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(mut a_string: String) -> String {
    a_string.push_str(", world");
    a_string // 返回 a_string 将所有权移出给调用的函数
}
