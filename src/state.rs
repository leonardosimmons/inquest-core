#![allow(unused)]
use std::hash::Hasher;

use bytes;
use linked_hash_map;

use crate::utils::Encryption;

type StateData = std::sync::Arc<
    Vec<
        std::sync::Mutex<
            linked_hash_map::LinkedHashMap<String, std::sync::Arc<std::sync::Mutex<bytes::Bytes>>>,
        >,
    >,
>;

struct State {
    current: StateData,
}
impl State {
    pub fn get(self, key: String) -> Option<bytes::Bytes> {
        let mut shard = self.get_shard(key.clone());
        match shard.get(&key[..]) {
            Some(val) => Some(val.lock().unwrap().clone()),
            None => None,
        }
    }

    pub fn insert(self, key: String, data: bytes::Bytes) {
        let mut shard = self.get_shard(key.clone());
        shard.insert(key, std::sync::Arc::new(std::sync::Mutex::new(data)));
    }

    pub fn new(capacity: usize) -> State {
        let mut v = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            v.push(std::sync::Mutex::new(linked_hash_map::LinkedHashMap::new()));
        }

        State {
            current: std::sync::Arc::new(v),
        }
    }

    fn get_shard(
        &self,
        key: String,
    ) -> std::sync::MutexGuard<
        linked_hash_map::LinkedHashMap<String, std::sync::Arc<std::sync::Mutex<bytes::Bytes>>>,
    > {
        let mut hash = Encryption::hash(key);
        self.current[usize::try_from(hash).unwrap() % self.current.len()]
            .lock()
            .unwrap()
    }
}
