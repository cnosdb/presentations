fn greet() {
    let cn = "你好，世界";
    let en = "hello, world";
    let lans = [cn, en];
    for lan in lans.iter() {
        println!("{}", &lan);
    }
}

macro_rules! say_hello {
    () => {
        println!("macro rule Hello!");
    };
}

fn test_fn() -> String {
    String::from("hello world")
}

fn main() {
    say_hello!();
    greet();
    println!("{}", test_fn())
}
