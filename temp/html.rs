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

pub trait RawHtml {
    fn document(&self) -> std::result::Result<Document, std::io::Error>;
}

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
}

impl RawHtml for Html {
    fn document(&self) -> std::result::Result<Document, std::io::Error> {
        Document::from_read(&**Arc::clone(&self.html).lock().unwrap())
    }
}

impl HtmlController for Html {
    fn descriptions(&self) -> Result<Vec<String>> {
        if let Ok(doc) = self.document() {
            Ok(doc
                .find(Name("meta"))
                .filter_map(|n| match n.attr("name") {
                    Some(name) => {
                        if name.contains("description") {
                            Some(name.to_string())
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }

    fn headers(&self, header: HtmlTag) -> Result<Headers> {
        match self.document() {
            Ok(doc) => match header {
                HtmlTag::H1 => Ok(Headers::new(
                    doc.find(Name("h1"))
                        .filter_map(|n| Some(n.text()))
                        .collect(),
                    1,
                )),
                HtmlTag::H2 => Ok(Headers::new(
                    doc.find(Name("h2"))
                        .filter_map(|n| Some(n.text()))
                        .collect(),
                    2,
                )),
                HtmlTag::H3 => Ok(Headers::new(
                    doc.find(Name("h3"))
                        .filter_map(|n| Some(n.text()))
                        .collect(),
                    3,
                )),
                HtmlTag::H4 => Ok(Headers::new(
                    doc.find(Name("h4"))
                        .filter_map(|n| Some(n.text()))
                        .collect(),
                    4,
                )),
                HtmlTag::H5 => Ok(Headers::new(
                    doc.find(Name("h5"))
                        .filter_map(|n| Some(n.text()))
                        .collect(),
                    5,
                )),
                HtmlTag::H6 => Ok(Headers::new(
                    doc.find(Name("h6"))
                        .filter_map(|n| Some(n.text()))
                        .collect(),
                    6,
                )),
                _ => Ok(Headers::Invalid(Vec::new())),
            },
            Err(err) => Err(Error::from(ErrorKind::Parse)),
        }
    }

    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
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

    fn page_title(&self) -> Result<Vec<String>> {
        if let Ok(doc) = self.document() {
            Ok(doc.find(Name("title")).map(|t| t.text()).collect())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }
}
