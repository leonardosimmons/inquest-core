#![allow(unused)]
use std::hash::Hasher;

use bytes;
use linked_hash_map;

use crate::utils::Encryption;

pub enum StateResponse {
    Data(bytes::Bytes),
    NoData,
}

type DataArray = linked_hash_map::LinkedHashMap<String, bytes::Bytes>;
type DataMatrix = std::sync::Arc<Vec<std::sync::Mutex<DataArray>>>;

#[derive(Clone)]
pub struct State {
    state: DataMatrix,
}
impl State {
    pub fn get(&self, key: &str) -> Option<bytes::Bytes> {
        let mut shard = self.get_shard(key.clone());
        match shard.get(key) {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    pub fn insert(&self, key: &str, data: bytes::Bytes) -> Option<bytes::Bytes> {
        let mut shard = self.get_shard(key.clone());
        match shard.insert(key.to_string(), data.clone()) {
            Some(val) => Some(val),
            None => None,
        }
    }

    pub fn new(capacity: usize) -> State {
        let mut v = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            v.push(std::sync::Mutex::new(linked_hash_map::LinkedHashMap::new()));
        }

        State {
            state: std::sync::Arc::new(v),
        }
    }

    pub fn remove(&self, key: &str) -> Option<bytes::Bytes> {
        let mut shard = self.get_shard(key.clone());
        match shard.remove(key) {
            Some(val) => Some(val),
            None => None
        }
    }

    fn get_shard(
        &self,
        key: &str,
    ) -> std::sync::MutexGuard<DataArray> {
        let mut hash = Encryption::hash(key);
        self.state[usize::try_from(hash).unwrap() % self.state.len()]
            .lock()
            .unwrap()
    }
}
