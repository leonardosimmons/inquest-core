use atoi::atoi;
use bytes::{Buf, Bytes};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::Cursor;
use std::vec;
use crate::error::{Error, ErrorKind};
use crate::parse::Parse;
use crate::utils::Result;

pub(crate) trait ByteController {
    /// Returns first byte in the buffer
    fn peek_byte(src: &mut std::io::Cursor<&[u8]>) -> Result<u8>;
    /// Returns single byte at cursor location
    fn read_byte(src: &mut std::io::Cursor<&[u8]>) -> Result<u8>;
    /// Skips buffer cursor to specified location
    fn skip_buffer(src: &mut std::io::Cursor<&[u8]>, n: usize) -> Result<()>;
    /// Reads a new-line terminated decimal
    fn read_newline_decimal(src: &mut std::io::Cursor<&[u8]>) -> Result<u64>;
    /// Returns single line of bytes starting at cursor location
    fn read_bytes_line<'a>(src: &mut std::io::Cursor<&'a [u8]>) -> Result<&'a [u8]>;
}

pub(crate) trait DataController {
    /// Creates and returns an empty `Data::Array`
    fn array() -> Data;
    /// Pushes bulk data into the data array
    fn push_bulk(&mut self, bytes: Bytes);
    /// Pushes int data into the data array
    fn push_int(&mut self, val: u64);
}

pub(crate) trait DataParser {
    /// Returns a data parser
    fn into_parts(self, data: Data) -> Result<Parse<DataChunk>>;
    /// Returns the next entry in the data array
    fn next(&mut self) -> Result<Data>;
    /// Returns the next entry in the data array as `Bytes`
    fn next_bytes(&mut self) -> Result<Bytes>;
    /// Returns the next entry in the data array as a `u64`
    fn next_int(&mut self) -> Result<u64>;
    /// Returns the next entry in the data array as a `String`
    fn next_string(&mut self) -> Result<String>;
    /// Ensures there aren't anymore entries within the data array
    fn finish(&mut self) -> Result<()>;
}

pub enum DataType {
    Html,
    Text,
}

#[derive(Serialize, Deserialize)]
pub enum Origin {
    Http,
    Path,
}

pub enum Data {
    Array(Vec<Data>),
    Bulk(Bytes),
    Error(String),
    Integer(u64),
    Null,
    Simple(String),
}

pub struct DataChunk {
    parts: vec::IntoIter<Data>
}

impl From<Data> for DataChunk {
    fn from(data: Data) -> Self {
        match data {
            Data::Array(d) => d.into(),
            data => panic!("protocol error; expecting a `Data::Array`, instead got {}", data)
        }
    }
}

impl From<Vec<Data>> for DataChunk {
    fn from(data: Vec<Data>) -> Self {
        Self { parts: data.into_iter() }
    }
}

impl ByteController for Data {
    fn peek_byte(src: &mut Cursor<&[u8]>) -> Result<u8> {
        if !src.has_remaining() {
            return Err(Error::from(ErrorKind::Unknown));
        }
        Ok(src.chunk()[0])
    }

    fn read_byte(src: &mut Cursor<&[u8]>) -> Result<u8> {
        if !src.has_remaining() {
            return Err(Error::from(ErrorKind::Unknown));
        }
        Ok(src.get_u8())
    }

    fn read_bytes_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8]> {
        let start = src.position() as usize;
        let end = src.get_ref().len() - 1;

        for i in start..end {
            if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
                src.set_position((i + 2) as u64);
                return Ok(&src.get_ref()[start..i]);
            }
        }

        Err(Error::from(ErrorKind::Parse))
    }

    fn read_newline_decimal(src: &mut Cursor<&[u8]>) -> Result<u64> {
        let line = Data::read_bytes_line(src)?;
        atoi::<u64>(line).ok_or_else(|| Error::from(ErrorKind::Parse))
    }

    fn skip_buffer(src: &mut Cursor<&[u8]>, n: usize) -> Result<()> {
        if !src.has_remaining() {
            return Err(Error::from(ErrorKind::Unknown));
        }
        Ok(src.advance(n))
    }
}

impl DataController for Data {
    fn array() -> Data {
        Data::Array(Vec::new())
    }

    fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Data::Array(arr) => arr.push(Data::Bulk(bytes)),
            _ => panic!("not a data array"),
        }
    }

    fn push_int(&mut self, val: u64) {
        match self {
            Data::Array(arr) => arr.push(Data::Integer(val)),
            _ => panic!("not a data array"),
        }
    }
}

impl Display for Data {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::str;

        match self {
            Data::Simple(response) => response.fmt(fmt),
            Data::Error(msg) => write!(fmt, "error: {}", msg),
            Data::Integer(num) => num.fmt(fmt),
            Data::Bulk(msg) => match str::from_utf8(msg) {
                Ok(string) => string.fmt(fmt),
                Err(_) => write!(fmt, "{:?}", msg),
            },
            Data::Null => "(nil)".fmt(fmt),
            Data::Array(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(fmt, " ")?;
                        part.fmt(fmt)?;
                    }
                }

                Ok(())
            }
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Html => write!(f, "html"),
            DataType::Text => write!(f, "text"),
        }
    }
}

impl Display for Origin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Origin::Http => write!(f, "http"),
            Origin::Path => write!(f, "path"),
        }
    }
}

impl PartialEq<&str> for Data {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Data::Simple(s) => s.eq(other),
            Data::Bulk(s) => s.eq(other),
            _ => false,
        }
    }
}

impl PartialEq<&str> for DataType {
    fn eq(&self, other: &&str) -> bool {
        match self {
            DataType::Html => "html".eq(*other),
            DataType::Text => "text".eq(*other),
        }
    }
}

impl PartialEq<&str> for Origin {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Origin::Http => "http".eq(*other),
            Origin::Path => "path".eq(*other),
        }
    }
}
