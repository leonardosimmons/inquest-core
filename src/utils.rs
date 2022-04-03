use std::result;

use crate::error::Error;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub(crate) trait Encryption {
    fn hash(key: &str) -> u64;
}

pub type Result<T> = result::Result<T, Error>;
