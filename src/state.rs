#![allow(unused)]
use std::hash::Hasher;

use bytes;
use linked_hash_map;

use crate::utils::{Encryption, Error};

const DEFAULT_STATE_ERROR_MSG: &str = "Unexpected error has occurred";

pub enum StateResponse {
    Data(bytes::Bytes),
    Error(Error),
    NotFound,
    Ok,
}

type DataArray = linked_hash_map::LinkedHashMap<String, bytes::Bytes>;
type DataMatrix = std::sync::Arc<Vec<std::sync::Mutex<DataArray>>>;

#[derive(Clone)]
pub struct State {
    state: DataMatrix,
}
impl State {
    pub fn get(&self, key: &str) -> StateResponse {
        let mut shard = self.get_shard(key.clone());
        match shard.get(key) {
            Some(val) => StateResponse::Data(val.clone()),
            None => StateResponse::NotFound,
            _ => StateResponse::Error(Error {
                msg: String::from(DEFAULT_STATE_ERROR_MSG),
                timestamp: chrono::Utc::now(),
            }),
        }
    }

    pub fn insert(&self, key: &str, data: bytes::Bytes) -> StateResponse {
        let mut shard = self.get_shard(key.clone());
        match shard.insert(key.to_string(), data.clone()) {
            Some(val) => StateResponse::Data(val),
            None => StateResponse::Ok,
            _ => StateResponse::Error(Error {
                msg: String::from(DEFAULT_STATE_ERROR_MSG),
                timestamp: chrono::Utc::now(),
            }),
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

    pub fn remove(&self, key: &str) -> StateResponse {
        let mut shard = self.get_shard(key.clone());
        match shard.remove(key) {
            Some(val) => StateResponse::Data(val),
            None => StateResponse::NotFound,
            _ => StateResponse::Error(Error {
                msg: String::from("key not found"),
                timestamp: chrono::Utc::now(),
            }),
        }
    }

    fn get_shard(&self, key: &str) -> std::sync::MutexGuard<DataArray> {
        let mut hash = State::hash(key);
        self.state[usize::try_from(hash).unwrap() % self.state.len()]
            .lock()
            .unwrap()
    }
}

impl Encryption for State {
    fn hash(key: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::ptr::hash(key, &mut hasher);
        hasher.finish()
    }
}
