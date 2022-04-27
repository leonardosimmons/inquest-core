use std::fmt::{Display, Formatter};
use bytes::Bytes;

pub enum Data {
    Array(Vec<Data>),
    Bulk(Bytes),
    Error(String),
    Integer(u64),
    Null,
    Simple(String),
}

pub enum DataType {
    Html,
    Text
}

pub enum Origin {
    Http,
    Path
}

impl Data {
    pub fn array() -> Data {
        Data::Array(Vec::new())
    }

    pub fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Data::Array(arr) => arr.push(Data::Bulk(bytes)),
            _ => panic!("not a data array")
        }
    }

    pub fn push_int(&mut self, val: u64) {
        match self {
            Data::Array(arr) => arr.push(Data::Integer(val)),
            _ => panic!("not a data array")
        }
    }
}

impl std::fmt::Display for Data {
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
            _ => false
        }
    }
}

impl PartialEq<&str> for DataType {
    fn eq(&self, other: &&str) -> bool {
        match self {
            DataType::Html => "html".eq(*other),
            DataType::Text => "text".eq(*other)
        }
    }
}

impl PartialEq<&str> for Origin {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Origin::Http => "http".eq(*other),
            Origin::Path => "path".eq(*other)
        }
    }
}
