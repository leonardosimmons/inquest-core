use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use bytes::Bytes;
use reqwest;
use select::document::Document;
use select::predicate::Predicate;

use crate::error::{Error, ErrorKind};
use crate::file::File;
use crate::parse::{FromPath, FromUrl};
use crate::utils::Result;

#[derive(Debug)]
pub enum HtmlAttribute {
    A,
    Content,
    Href,
    Name,
}

impl Display for HtmlAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlAttribute::A => write!(f, "a"),
            HtmlAttribute::Content => write!(f, "content"),
            HtmlAttribute::Href => write!(f, "href"),
            HtmlAttribute::Name => write!(f, "name"),
        }
    }
}

impl From<HtmlAttribute> for &str {
    fn from(attr: HtmlAttribute) -> Self {
        match attr {
            HtmlAttribute::A => "a",
            HtmlAttribute::Content => "content",
            HtmlAttribute::Href => "href",
            HtmlAttribute::Name => "name",
        }
    }
}

impl FromStr for HtmlAttribute {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "a" => Ok(HtmlAttribute::A),
            "content" => Ok(HtmlAttribute::Content),
            "href" => Ok(HtmlAttribute::Href),
            "name" => Ok(HtmlAttribute::Name),
            _ => Err(Error::from(ErrorKind::InvalidParameters)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HtmlTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Invalid,
    Meta,
    Title,
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
            HtmlTag::Meta => write!(f, "meta"),
            HtmlTag::Title => write!(f, "title"),
        }
    }
}

impl FromStr for HtmlTag {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "1" => Ok(HtmlTag::H1),
            "2" => Ok(HtmlTag::H2),
            "3" => Ok(HtmlTag::H3),
            "4" => Ok(HtmlTag::H4),
            "5" => Ok(HtmlTag::H5),
            "6" => Ok(HtmlTag::H6),
            "invalid" => Ok(HtmlTag::Invalid),
            "meta" => Ok(HtmlTag::Meta),
            "title" => Ok(HtmlTag::Title),
            _ => Err(Error::from(ErrorKind::InvalidParameters)),
        }
    }
}

impl From<HtmlTag> for &str {
    fn from(tag: HtmlTag) -> Self {
        match tag {
            HtmlTag::H1 => "1",
            HtmlTag::H2 => "2",
            HtmlTag::H3 => "3",
            HtmlTag::H4 => "4",
            HtmlTag::H5 => "5",
            HtmlTag::H6 => "6",
            HtmlTag::Invalid => "invalid",
            HtmlTag::Meta => "meta",
            HtmlTag::Title => "title",
        }
    }
}

impl From<HtmlTag> for u8 {
    fn from(tag: HtmlTag) -> Self {
        match tag {
            HtmlTag::H1 => 1u8,
            HtmlTag::H2 => 2u8,
            HtmlTag::H3 => 3u8,
            HtmlTag::H4 => 4u8,
            HtmlTag::H5 => 5u8,
            HtmlTag::H6 => 6u8,
            _ => 0u8,
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
            _ => Headers::Invalid(Vec::new()),
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
    fn header(&self, header: HtmlTag) -> Result<Headers>;
    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>>;
    fn page_title(&self) -> Result<Vec<String>>;
}

pub struct Html {
    html: Arc<Mutex<Bytes>>,
}

impl Default for Html {
    fn default() -> Self {
        Self {
            html: Arc::new(Mutex::new(Bytes::from(""))),
        }
    }
}

#[async_trait]
impl FromPath for Html {
    async fn from(&mut self, path: &str, buf: String) -> Result<Self> {
        Ok(Self {
            html: Arc::new(Mutex::new(Bytes::from(
                File::from(path, buf).await?,
            ))),
        })
    }
}

#[async_trait]
impl FromUrl for Html {
    async fn from(&mut self, url: &str) -> Result<Self> {
        match reqwest::get(url).await {
            Ok(resp) => {
                if let Ok(doc) = resp.text().await {
                    Ok(Self {
                        html: Arc::new(Mutex::new(Bytes::from(doc))),
                    })
                } else {
                    Err(Error::from(ErrorKind::Html))
                }
            }
            Err(_) => Err(Error::from(ErrorKind::Http)),
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
            Err(_) => Err(Error::from(ErrorKind::Document)),
        }
    }

    fn text(&self) -> Result<String> {
        match str::from_utf8(&**Arc::clone(&self.html).lock().unwrap()) {
            Ok(text) => Ok(text.to_string()),
            Err(_) => Err(Error::from(ErrorKind::Parse)),
        }
    }
}
