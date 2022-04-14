#![allow(unused)]
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

use bytes::Bytes;
use select::document::Document;
use select::predicate::Predicate;

use crate::error::{Error, ErrorKind};
use crate::html::{Headers, Html, HtmlController, HtmlDocument, HtmlParser, HtmlTag};
use crate::utils::Result;

pub struct Parse<T> {
    parse: T,
}

impl<T> Parse<T> {
    pub fn new(kind: T) -> Parse<T> {
        Parse { parse: kind }
    }

    pub fn path_to_string(paths: PathBuf) -> String {
        paths.to_str().unwrap_or_default().to_string()
    }
}

impl Parse<Html> {
    pub fn from(document: &str) -> Result<Parse<Html>> {
        Ok(Parse {
            parse: Html::new(document),
        })
    }

    pub async fn from_url(html: &str) -> Result<Parse<Html>> {
        Ok(Parse::new(Html::from_url(html).await?))
    }

    /// Trims `https://` from the start and `/` from the end of the link (if applicable)
    pub fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches(":")
            .trim_start_matches("//")
            .trim_end_matches('/')
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

impl HtmlController for Parse<Html> {
    fn descriptions(&self) -> Result<Vec<String>> {
        self.parse.descriptions()
    }

    fn headers(&self, header: HtmlTag) -> Result<Headers> {
        match self.document() {
            _doc => match header {
                HtmlTag::H1 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 1))
                        .unwrap_or_else(|_| Vec::new()),
                    1,
                )),
                HtmlTag::H2 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 2))
                        .unwrap_or_else(|_| Vec::new()),
                    2,
                )),
                HtmlTag::H3 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 3))
                        .unwrap_or_else(|_| Vec::new()),
                    3,
                )),
                HtmlTag::H4 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 4))
                        .unwrap_or_else(|_| Vec::new()),
                    4,
                )),
                HtmlTag::H5 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 5))
                        .unwrap_or_else(|_| Vec::new()),
                    5,
                )),
                HtmlTag::H6 => Ok(Headers::new(
                    self.parse
                        .headers(&format!("h{}", 6))
                        .unwrap_or_else(|_| Vec::new()),
                    6,
                )),
                _ => Err(Error::from(ErrorKind::InvalidHtmlTag)),
            },
        }
    }

    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        self.parse.links(predicate)
    }

    fn page_title(&self) -> Result<Vec<String>> {
        self.parse.title()
    }
}
