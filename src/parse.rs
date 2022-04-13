#![allow(unused)]
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

use bytes::Bytes;
use select::document::Document;
use select::predicate::{Name, Predicate};

use crate::error::{Error, ErrorKind};
use crate::html::{Headers, Html, HtmlParser, HtmlTag};
use crate::utils::{Responder, Result};

pub struct Parse<T> {
    parse: T,
}

impl<T> Parse<T> {
    pub fn new(kind: T) -> Parse<T> {
        Parse { parse: kind }
    }
}

impl Parse<Html> {
    pub(crate) fn from(document: &str) -> Result<Parse<Html>> {
        Ok(Parse {
            parse: Html::new(document),
        })
    }

    pub(crate) async fn from_url(html: &str) -> Result<Parse<Html>> {
        Ok(Parse::new(Html::from_url(html).await?))
    }

    pub(crate) fn path_to_string(paths: PathBuf) -> String {
        paths
            .to_str()
            .unwrap_or_default()
            .to_string()
    }

    pub(crate) async fn all_headers(&self, mut buff: Vec<Headers>) -> Result<Vec<Headers>> {
        use HtmlTag::*;

        for i in 1..=6 {
            match i {
                1 => buff.push(self.headers(H1).await?),
                2 => buff.push(self.headers(H2).await?),
                3 => buff.push(self.headers(H3).await?),
                4 => buff.push(self.headers(H4).await?),
                5 => buff.push(self.headers(H5).await?),
                6 => buff.push(self.headers(H6).await?),
                _ => {}
            }
        }
        Ok(buff)
    }

    pub(crate) async fn all_links(&self) -> Result<Vec<String>> {
        self.parse.links(Name("a"))
    }

    /// Trims `https://` from the start and `/` from the end of the link (if applicable)
    pub(crate) fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches(":")
            .trim_start_matches("//")
            .trim_end_matches('/')
    }

    pub(crate) async fn headers(&self, header: HtmlTag) -> Result<Headers> {
        match self.document() {
            _doc => match header {
                HtmlTag::H1 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 1))
                        .unwrap_or_else(|_| Vec::new()),
                    1
                )),
                HtmlTag::H2 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 2))
                        .unwrap_or_else(|_| Vec::new()),
                    2
                )),
                HtmlTag::H3 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 3))
                        .unwrap_or_else(|_| Vec::new()),
                    3
                )),
                HtmlTag::H4 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 4))
                        .unwrap_or_else(|_| Vec::new()),
                    4
                )),
                HtmlTag::H5 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 5))
                        .unwrap_or_else(|_| Vec::new()),
                    5
                )),
                HtmlTag::H6 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 6))
                        .unwrap_or_else(|_| Vec::new()),
                    6
                )),
                _ => Err(Error::from(ErrorKind::InvalidHtmlTag))
            },
        }
    }

    pub(crate) async fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        self.parse.links(predicate)
    }

    pub(crate) async fn page_title(&self) -> Result<Vec<String>> {
        self.parse.title()
    }
}

impl HtmlParser for Parse<Html> {
    fn bytes(&self) -> Bytes {
        Arc::clone(&self.parse.html).lock().unwrap().clone()
    }

    fn document(&self) -> Result<Document> {
        if let Ok(doc) = Document::from_read(&**Arc::clone(&self.parse.html).lock().unwrap()) {
            Ok(doc)
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }

    fn text(&self) -> Result<String> {
        if let Ok(text) = str::from_utf8(&*Arc::clone(&self.parse.html).lock().unwrap()) {
            Ok(text.to_owned())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }
}
