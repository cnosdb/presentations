use tokio;

#[tokio::main]
async fn main() {
    use futures::channel::oneshot;
    use futures::stream::{self, StreamExt, TryStreamExt};

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    let (_tx3, rx3) = oneshot::channel();

    let stream = stream::iter(vec![rx1, rx2, rx3]);
    let fut = stream.map(Ok).try_for_each_concurrent(
        /* limit */ 2,
        |rx| async move {
            let res: Result<(), oneshot::Canceled> = rx.await;
            res
        }
    );

    tx1.send(()).unwrap();
// Drop the second sender so that `rx2` resolves to `Canceled`.
    drop(tx2);

// The final result is an error because the second future
// resulted in an error.
    assert_eq!(Err(oneshot::Canceled), fut.await)
}