use std::future::Future;
use futures::executor::block_on;

fn bad() -> impl Future<Output = u8> {
    let x = 5;
    borrow_x(&x) // ERROR: `x` does not live long enough
}

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

async fn borrow_x(x: &u8) -> u8 { *x }

fn main() {
    block_on(bad());
}