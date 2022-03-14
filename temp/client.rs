#![allow(unused)]

use bytes::Bytes;
use mini_redis::client;
use std::str::from_utf8;
use tokio::sync::oneshot;

// OneShot -> single producer, single consumer
// NOTE capacity ALWAYS == 1 & CAN NOT be cloned

// Provided by the requester and used by the manager task
// to send the command response back to the requester
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<Option<Bytes>>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    // ignore errors
                    // sends response through channel back to task
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    client.set(&key, val).await;
                    let _ = resp.send(mini_redis::Result::Ok(Some(Bytes::from("Ok"))));
                }
            }
        }
    });
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: String::from("foo"),
            resp: resp_tx,
        };

        // send GET request to manager
        tx.send(cmd).await.unwrap();

        // await response from manager
        let res = resp_rx.await;
        println!(
            "Get Response: {:?}",
            std::str::from_utf8(&res.unwrap().unwrap().unwrap().to_vec()).unwrap()
        );
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: String::from("foo"),
            val: Bytes::from("bar"),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!(
            "Set Response: {:?}",
            std::str::from_utf8(&res.unwrap().unwrap().unwrap().to_vec()).unwrap()
        );
    });

    t2.await.unwrap();
    t1.await.unwrap();
    manager.await.unwrap();
}
