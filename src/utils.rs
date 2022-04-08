use crate::error::Error;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub(crate) trait Encryption {
    fn hash(key: &str) -> u64;
}

pub type Responder<T> = tokio::sync::oneshot::Sender<Result<T>>;

pub type Result<T> = std::result::Result<T, Error>;
