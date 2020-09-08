use futures::stream::FuturesUnordered;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let (tx, rx) = flume::unbounded();
    tokio::spawn(async move {
        let n_sends = 100000;
        for _ in 0..n_sends {
            tx.send_async(()).await.unwrap();
            // tx.send().unwrap() is OK
        }
        println!("send end");
    });

    let mut futures_unordered = (0..250)
        .map(|_| async {
            while let Ok(()) = rx.recv_async().await
            /* rx.recv() is OK */
            {}
        })
        .collect::<FuturesUnordered<_>>();

    while futures_unordered.next().await.is_some() {}
    println!("recv end");
}
