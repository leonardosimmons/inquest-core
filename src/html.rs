use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use reqwest;
use select::document::Document;
use select::predicate::Predicate;

use crate::error::{Error, ErrorKind};
use crate::file::File;
use crate::utils::Result;

#[derive(Debug)]
pub enum HtmlTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Invalid,
}

impl Display for HtmlTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlTag::H1 => write!(f, "h1"),
            HtmlTag::H2 => write!(f, "h2"),
            HtmlTag::H3 => write!(f, "h3"),
            HtmlTag::H4 => write!(f, "h4"),
            HtmlTag::H5 => write!(f, "h5"),
            HtmlTag::H6 => write!(f, "h6"),
            HtmlTag::Invalid => write!(f, "invalid"),
        }
    }
}

pub enum Headers {
    H1(Vec<String>),
    H2(Vec<String>),
    H3(Vec<String>),
    H4(Vec<String>),
    H5(Vec<String>),
    H6(Vec<String>),
    Invalid(Vec<String>),
}
impl Headers {
    pub fn new(headers: Vec<String>, index: u8) -> Headers {
        match index {
            1 => Headers::H1(headers),
            2 => Headers::H2(headers),
            3 => Headers::H3(headers),
            4 => Headers::H4(headers),
            5 => Headers::H5(headers),
            6 => Headers::H6(headers),
            _ => Headers::Invalid(vec![String::from("")]),
        }
    }
}
impl Deref for Headers {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        match self {
            Headers::H1(h) => h,
            Headers::H2(h) => h,
            Headers::H3(h) => h,
            Headers::H4(h) => h,
            Headers::H5(h) => h,
            Headers::H6(h) => h,
            Headers::Invalid(h) => h,
        }
    }
}

pub trait HtmlDocument {
    fn bytes(&self) -> Bytes;
    fn document(&self) -> Result<Document>;
    fn text(&self) -> Result<String>;
}

pub trait HtmlParser {
    fn descriptions(&self) -> Result<Vec<String>>;
    fn headers(&self, header: HtmlTag) -> Result<Headers>;
    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>>;
    fn page_title(&self) -> Result<Vec<String>>;
}

pub struct Html {
    pub html: Arc<Mutex<Bytes>>,
}

impl Html {
    pub async fn from(path: &str, buf: &[u8]) -> Result<Html> {
        Ok(Html {
            html: Arc::new(Mutex::new(Bytes::from(File::from(path, buf).await?))),
        })
    }

    pub async fn from_url(url: &str) -> Result<Html> {
        match reqwest::get(url).await {
            Ok(resp) => {
                if let Ok(doc) = resp.text().await {
                    Ok(Html {
                        html: Arc::new(Mutex::new(Bytes::from(doc))),
                    })
                } else {
                    Err(Error::from(ErrorKind::Html))
                }
            }
            Err(_) => Err(Error::from(ErrorKind::Failed)),
        }
    }
}

impl HtmlDocument for Html {
    fn bytes(&self) -> Bytes {
        Arc::clone(&self.html).lock().unwrap().to_vec().into()
    }

    fn document(&self) -> Result<Document> {
        match Document::from_read(&**Arc::clone(&self.html).lock().unwrap()) {
            Ok(doc) => Ok(doc),
            Err(_) => Err(Error::from(ErrorKind::Parse)),
        }
    }

    fn text(&self) -> Result<String> {
        match str::from_utf8(&**Arc::clone(&self.html).lock().unwrap()) {
            Ok(text) => Ok(text.to_owned()),
            Err(_) => Err(Error::from(ErrorKind::Parse)),
        }
    }
}

impl Default for Html {
    fn default() -> Self {
        Self {
            html: Arc::new(Mutex::new(Bytes::from(""))),
        }
    }
}
