use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

use crate::error::{Error, ErrorKind};
use crate::utils::Result;

pub struct File {
    text: String,
}

impl File {
    pub async fn from(path: &str) -> Result<File> {
        match TokioFile::open(path).await {
            Ok(mut f) => {
                let mut buffer = String::new();
                if let Ok(_) = f.read_to_string(&mut buffer).await {
                    Ok(File { text: buffer })
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
