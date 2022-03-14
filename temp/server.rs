#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;
use std::ptr::hash;

use bytes::Bytes;
use mini_redis::{Connection, Frame};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);

    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379")
        .await
        .unwrap();

    println!("Listening");
    let db = new_sharded_db(5);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: tokio::net::TcpStream, db: ShardedDb) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut hasher = DefaultHasher::new();
                std::ptr::hash(cmd.key(), &mut hasher);
                let mut db = db[usize::try_from(hasher.finish()).unwrap() % db.len()]
                    .lock()
                    .unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("Okay".to_string())
            }
            Get(cmd) => {
                let mut hasher = DefaultHasher::new();
                std::ptr::hash(cmd.key(), &mut hasher);
                let mut db = db[usize::try_from(hasher.finish()).unwrap() % db.len()]
                    .lock()
                    .unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
