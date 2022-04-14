#![allow(unused)]

use std::path::PathBuf;
use crate::error::Error;
use crate::parse::{Parse, Parser};
use crate::probe::Probe;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub(crate) trait Encryption {
    fn hash(key: &str) -> u64;
}

pub(crate) type Responder<T> = tokio::sync::oneshot::Sender<Result<T>>;

pub type Result<T> = std::result::Result<T, Error>;


pub struct Unknown;
impl Parser for Unknown {
    fn path_to_string(paths: PathBuf) -> String {
        paths.to_str().unwrap_or_default().to_string()
    }
}
impl Probe for Unknown {}