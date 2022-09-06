use std::future::Future;
use futures::executor::block_on;

async fn hello_world0() {
    println!("hello, world0!");
}

fn hello_world1() -> impl Future<Output=()> {
    async {
        println!("hello,world1!");
    }
}


fn main() {
    let future = hello_world0();
    block_on(future);
    let future = hello_world1();
    block_on(future);
}

