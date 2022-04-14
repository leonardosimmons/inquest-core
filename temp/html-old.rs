#![allow(unused)]
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use reqwest;
use select::document::Document;
use select::predicate::{Name, Predicate};

use crate::error::{Error, ErrorKind};
use crate::parse::Parse;
use crate::utils::Result;
pub enum HtmlTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Invalid,
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
    pub(crate) fn new(headers: Vec<String>, index: u8) -> Headers {
        match index {
            1 => Headers::H1(headers),
            2 => Headers::H2(headers),
            3 => Headers::H3(headers),
            4 => Headers::H4(headers),
            5 => Headers::H5(headers),
            6 => Headers::H6(headers),
            _ => Headers::Invalid(vec!["".to_string()]),
        }
    }
}
impl Deref for Headers {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        match self {
            Headers::H1(heads) => heads,
            Headers::H2(heads) => heads,
            Headers::H3(heads) => heads,
            Headers::H4(heads) => heads,
            Headers::H5(heads) => heads,
            Headers::H6(heads) => heads,
            Headers::Invalid(heads) => heads
        }
    }
}

pub trait HtmlDocument {}

pub(crate) trait HtmlParser {
    fn bytes(&self) -> Bytes;
    fn document(&self) -> Result<Document>;
    fn text(&self) -> Result<String>;
}

pub(crate) trait HtmlController {
    fn descriptions(&self) -> Result<Vec<String>>;
    fn headers(&self, header: HtmlTag) -> Result<Headers>;
    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>>;
    fn page_title(&self) -> Result<Vec<String>>;
}

pub struct Html {
    pub(crate) html: Arc<Mutex<Bytes>>,
}

impl Html {
    pub(crate) fn new(document: &str) -> Html {
        Html {
            html: Arc::new(Mutex::new(Bytes::from(document.to_string()))),
        }
    }

    pub(crate) async fn from_url(url: &str) -> Result<Html> {
        match reqwest::get(url).await {
            Ok(resp) => {
                if let Ok(doc) = resp.text().await {
                    Ok(Html::new(&doc))
                } else {
                    Err(Error::from(ErrorKind::Html))
                }
            }
            Err(_) => Err(Error::from(ErrorKind::Failed)),
        }
    }

    pub(crate) fn document(&self) -> std::result::Result<Document, std::io::Error> {
        Document::from_read(&**Arc::clone(&self.html).lock().unwrap())
    }

    pub(crate) fn descriptions(&self) -> Result<Vec<String>> {
        if let Ok(doc) = self.document() {
            Ok(doc
                .find(Name("meta"))
                .filter_map(|n| {
                    match n.attr("name") {
                        Some(name) => {
                            if name.contains("description") {
                                Some(name.to_string())
                            } else {
                                None
                            }
                        }
                        None => None
                    }
                })
                .collect())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }

    pub(crate) fn headers(&self, tag: &str) -> Result<Vec<String>> {
        if let Ok(doc) = self.document() {
            Ok(doc.find(Name(tag)).filter_map(|n| Some(n.text())).collect())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }

    pub(crate) fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        if let Ok(doc) = self.document() {
            Ok(doc
                .find(predicate)
                .filter_map(|n| {
                    if let Some(link) = n.attr("href") {
                        Some(Parse::fix_link(link))
                    } else {
                        None
                    }
                })
                .map(|x| x.to_string())
                .collect())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }

    pub(crate) fn title(&self) -> Result<Vec<String>> {
        if let Ok(doc) = self.document() {
            Ok(doc.find(Name("title")).map(|t| t.text()).collect())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }
}

impl HtmlDocument for Html {}