use std::hash::Hasher;

use chrono;

pub trait Encryption {
    fn hash(key: &str) -> u64;
}

pub struct Error {
    pub msg: String,
    pub timestamp: DateTime
}


pub type DateTime = chrono::DateTime<chrono::Utc>;
