use std::hash::Hasher;

use chrono;

pub struct Encryption;
impl Encryption {
    pub fn hash(key: String) -> u64 {
        let key = &key[..];
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::ptr::hash(key, &mut hasher);
        hasher.finish()
    }
}

pub type Time = chrono::DateTime<chrono::Utc>;
