use std::hash::Hasher;

use chrono;

pub struct Encryption;
impl Encryption {
    pub fn hash(key: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::ptr::hash(key, &mut hasher);
        hasher.finish()
    }
}

pub type Time = chrono::DateTime<chrono::Utc>;
