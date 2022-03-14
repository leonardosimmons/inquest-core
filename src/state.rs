#![allow(unused)]
use bytes;
use chrono;
use linked_hash_map;

const DEFAULT_STATE_SHARD_CAPACITY: usize = 20;
const MAX_STATE_SHARD_CAPACITY: usize = 100;

type StateData =
std::sync::Arc<Vec<std::sync::Mutex<linked_hash_map::LinkedHashMap<String, bytes::Bytes>>>>;

struct State {
    current: StateData,
}
impl State {
    pub fn new() -> State {
        State {
            current: std::sync::Arc::new(Vec::new()),
        }
    }

    pub fn capacity(self, num_shards: usize) -> State {
        match num_shards > self.current.len() {
            true => {
                match num_shards < MAX_STATE_SHARD_CAPACITY {
                    true => {
                        let mut v = Vec::with_capacity(num_shards);

                        for item in &*self.current {
                            v.push(std::sync::Mutex::new(item.lock().unwrap().clone()))
                        }

                        for _ in v.len() - 1..num_shards {
                            v.push(std::sync::Mutex::new(linked_hash_map::LinkedHashMap::new()));
                        }

                        State {
                            current: std::sync::Arc::new(v),
                        }
                    },
                    false => {
                        println!("Unable to change state capacity, value must be smaller than {}", MAX_STATE_SHARD_CAPACITY);
                        self
                    }
                }
            }
            false => {
                println!("Unable to change state capacity, value must be larger than current size");
                self
            }
        }
    }
}
