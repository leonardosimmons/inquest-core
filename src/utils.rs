#![allow(unused)]
use crate::error::Error;
use crate::probe::FileProbe;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub(crate) trait Encryption {
    fn hash(key: &str) -> u64;
}

pub(crate) type Responder<T> = tokio::sync::oneshot::Sender<Result<T>>;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Unknown;
impl FileProbe for Unknown {}