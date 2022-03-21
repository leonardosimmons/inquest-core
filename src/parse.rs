#![allow(unused)]
use std::sync::{Arc, Mutex};

use bytes::Bytes;

#[derive(Debug)]
enum ParseError {
    EndOfStream,
    Other(String)
}

struct Parse {
    document: Arc<Mutex<Bytes>>,
}
impl Parse {
    fn new(document: String) -> Parse {
        Parse {
            document: Arc::new(Mutex::new(document.into()))
        }
    }
}
