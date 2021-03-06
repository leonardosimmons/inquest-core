use std::str;

use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

use crate::error::{Error, ErrorKind};
use crate::utils::Result;

pub struct File {
    text: String,
}

impl File {
    pub async fn new(path: &str, capacity: usize) -> File {
        File {
            text: File::from(path, String::with_capacity(capacity)).await.unwrap(),
        }
    }

    pub async fn from(path: &str, mut buf: String) -> Result<String> {
        match TokioFile::open(path).await {
            Ok(mut f) => {
                if let Ok(_) = f.read_to_string(&mut buf).await {
                    Ok(buf)
                } else {
                    Err(Error::from(ErrorKind::InvalidUtf8))
                }
            }
            Err(_) => Err(Error::from(ErrorKind::NotFound)),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Default for File {
    fn default() -> Self {
        File { text: String::new() }
    }
}
