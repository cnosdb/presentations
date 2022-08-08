use std::future::Future;
use std::io;
use std::pin::Pin;
use futures::executor::block_on;
use futures::Stream;

fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{my_string}");
    }
}

fn main() {
    block_on(move_block());
}
