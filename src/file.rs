#![allow(unused)]
use std::io::Bytes;
use std::str;

use bytes::BytesMut;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

use crate::error::{Error, ErrorKind};
use crate::utils::Result;

pub struct File {
    text: String,
}

impl File {
    pub async fn new(path: &str, mut buf: &[u8]) -> File {
        File {
            text: File::from(path, buf).await.unwrap(),
        }
    }

    pub async fn from(path: &str, mut buf: &[u8]) -> Result<String> {
        match TokioFile::open(path).await {
            Ok(mut f) => {
                if let Ok(_) = f.read_to_string(&mut str::from_utf8(buf).unwrap().to_string()).await {
                    Ok(str::from_utf8(buf).unwrap().to_string())
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