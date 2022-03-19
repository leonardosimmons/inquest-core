use std::hash::Hasher;

use chrono;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub trait Encryption {
    fn hash(key: &str) -> u64;
}

pub struct Error {
    pub msg: String,
    pub timestamp: DateTime,
}
