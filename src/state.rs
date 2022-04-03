use std::hash::Hasher;
use std::sync::{Arc, Mutex, MutexGuard};

use bytes::Bytes;
use chrono::Utc;
use linked_hash_map::LinkedHashMap;

use crate::utils::{DateTime, Encryption};

#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub timestamp: DateTime,
}

pub enum StateResponse {
    Data(Bytes),
    Error(Error),
    NotFound,
    Ok,
}

type DataArray = LinkedHashMap<String, Bytes>;
type DataMatrix = Arc<Vec<Mutex<DataArray>>>;

#[derive(Clone)]
pub struct State {
    state: DataMatrix,
}
impl State {
    pub fn back(&mut self, key: &str) -> StateResponse {
        let mut shard = self.get_shard(key);
        if let Some((_, data)) = shard.back() {
            StateResponse::Data(data.clone())
        } else {
            StateResponse::Error(Error {
                msg: String::from("Unable to retrieve last element"),
                timestamp: Utc::now()
            })
        }
    }

    pub fn front(&mut self, key: &str) -> StateResponse {
        let shard = self.get_shard(key);
        if let Some((_, data)) = shard.front() {
            StateResponse::Data(data.clone())
        } else {
            StateResponse::Error(Error {
                msg: String::from("Unable to retrieve first element"),
                timestamp: Utc::now()
            })
        }
    }

    pub fn get(&self, key: &str) -> StateResponse {
        let shard = self.get_shard(key.clone());
        match shard.get(key) {
            Some(val) => StateResponse::Data(val.clone()),
            None => StateResponse::NotFound,
        }
    }

    pub fn insert(&self, key: &str, data: Bytes) -> StateResponse {
        let mut shard = self.get_shard(key.clone());
        match shard.insert(key.to_string(), data.clone()) {
            Some(val) => StateResponse::Data(val),
            None => StateResponse::Ok,
        }
    }

    pub fn new(capacity: usize) -> State {
        let mut v = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            v.push(Mutex::new(LinkedHashMap::new()));
        }

        State {
            state: Arc::new(v),
        }
    }

    pub fn pop_back(&mut self, key: &str) -> StateResponse {
        let mut shard = self.get_shard(key);
        match shard.pop_back() {
            Some((_, data)) => StateResponse::Data(data),
            None => StateResponse::Error(Error {
                msg: String::from("Unable to remove last element"),
                timestamp: Utc::now()
            })
        }
    }

    pub fn pop_front(&mut self, key: &str) -> StateResponse {
        let mut shard = self.get_shard(key);
        match shard.pop_front() {
            Some((_, data)) => StateResponse::Data(data),
            None => StateResponse::Error(Error {
                msg: String::from("Unable to remove first element"),
                timestamp: Utc::now()
            })
        }
    }

    pub fn remove(&self, key: &str) -> StateResponse {
        let mut shard = self.get_shard(key.clone());
        match shard.remove(key) {
            Some(val) => StateResponse::Data(val),
            None => StateResponse::NotFound,
        }
    }

    fn get_shard(&self, key: &str) -> MutexGuard<DataArray> {
        let hash = State::hash(key);
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
