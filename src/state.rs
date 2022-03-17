#![allow(unused)]
use std::hash::Hasher;

use bytes;
use linked_hash_map;

use crate::global::Time;

const DEFAULT_STATE_SHARD_CAPACITY: usize = 20;
const MAX_STATE_SHARD_CAPACITY: usize = 100;

type StateData = std::sync::Arc<
    Vec<
        std::sync::Mutex<
            linked_hash_map::LinkedHashMap<String, std::sync::Arc<std::sync::Mutex<bytes::Bytes>>>,
        >,
    >,
>;

struct Status {
    expires: Option<Time>,
}

struct State {
    current: StateData,
    status: Status,
}
impl State {
    pub fn get(self, key: &str) -> Option<bytes::Bytes> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::ptr::hash(key, &mut hasher);

        let mut shard = self.current
            [usize::try_from(hasher.finish()).unwrap() % self.current.len()]
        .lock()
        .unwrap();

        let data = match shard.get(key).unwrap().lock().unwrap() {
            val => Some(val.clone()),
            _ => None,
        };
        data
    }

    pub fn new(capacity: usize) -> State {
        let mut v = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            v.push(std::sync::Mutex::new(linked_hash_map::LinkedHashMap::new()));
        }

        State {
            current: std::sync::Arc::new(v),
            status: Status { expires: None },
        }
    }
}
