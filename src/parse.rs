#![allow(unused)]
use std::str;

use bytes::Bytes;
use select::document::Document;
use select::predicate::{Name, Predicate};

use crate::error::{Error, ErrorKind};
use crate::html::{Headers, Html, HtmlTag};
use crate::utils::Result;

pub struct Parse<T> {
    parse: T,
}

impl<T> Parse<T> {
    pub fn new(kind: T) -> Parse<T> {
        Parse { parse: kind }
    }
}

impl Parse<Html> {
    pub fn from(document: String) -> Parse<Html> {
        Parse {
            parse: Html::new(document),
        }
    }

    pub async fn from_url(html: &str) -> Result<Parse<Html>> {
        Ok(Parse::new(Html::from_url(html).await?))
    }

    pub(crate) fn bytes(&self) -> Bytes {
        self.parse.html.lock().unwrap().clone()
    }

    pub(crate) async fn document(&self) -> Result<Document> {
        if let Ok(doc) = Document::from_read(&*self.parse.html.lock().unwrap().clone()) {
            Ok(doc)
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }

    pub(crate) fn text(&self) -> Result<String> {
        if let Ok(text) = str::from_utf8(&*self.parse.html.lock().unwrap().clone()) {
            Ok(text.to_owned())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }

    pub(crate) async fn all_headers(&self) -> Result<Vec<Headers>> {
        use HtmlTag::*;

        let mut buff: Vec<_> = vec![];
        for i in 1..=6 {
            match i {
                1 => buff.push(self.header(H1).await?),
                2 => buff.push(self.header(H2).await?),
                3 => buff.push(self.header(H3).await?),
                4 => buff.push(self.header(H4).await?),
                5 => buff.push(self.header(H5).await?),
                6 => buff.push(self.header(H6).await?),
                _ => {}
            }
        }
        Ok(buff)
    }

    pub(crate) async fn all_links(&self) -> Result<Vec<String>> {
        self.parse.links(Name("a"))
    }

    pub(crate) async fn header(&self, header: HtmlTag) -> Result<Headers> {
        match self.document().await {
            _doc => match header {
                HtmlTag::H1 => Ok(Parse::headers_from_tag(
                    1,
                    self.parse
                        .headers(&format!("h{}", 1))
                        .unwrap_or_else(|_| Vec::new()),
                )),
                HtmlTag::H2 => Ok(Parse::headers_from_tag(
                    2,
                    self.parse
                        .headers(&format!("h{}", 2))
                        .unwrap_or_else(|_| Vec::new()),
                )),
                HtmlTag::H3 => Ok(Parse::headers_from_tag(
                    3,
                    self.parse
                        .headers(&format!("h{}", 3))
                        .unwrap_or_else(|_| Vec::new()),
                )),
                HtmlTag::H4 => Ok(Parse::headers_from_tag(
                    4,
                    self.parse
                        .headers(&format!("h{}", 4))
                        .unwrap_or_else(|_| Vec::new()),
                )),
                HtmlTag::H5 => Ok(Parse::headers_from_tag(
                    5,
                    self.parse
                        .headers(&format!("h{}", 5))
                        .unwrap_or_else(|_| Vec::new()),
                )),
                HtmlTag::H6 => Ok(Parse::headers_from_tag(
                    6,
                    self.parse
                        .headers(&format!("h{}", 6))
                        .unwrap_or_else(|_| Vec::new()),
                )),
            },
        }
    }

    pub(crate) async fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        self.parse.links(predicate)
    }

    pub(crate) async fn page_title(&self) -> Result<Vec<String>> {
        self.parse.title()
    }

    /// Trims `https://` from the start and `/` from the end of the link (if applicable)
    pub(crate) fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches(":")
            .trim_start_matches("//")
            .trim_end_matches('/')
    }

    pub(crate) fn headers(headers: &Headers) -> Vec<String> {
        let mut buff = vec![];
        match headers {
            Headers::H1(heads) => heads.iter().for_each(|h| buff.push(h.clone())),
            Headers::H2(heads) => heads.iter().for_each(|h| buff.push(h.clone())),
            Headers::H3(heads) => heads.iter().for_each(|h| buff.push(h.clone())),
            Headers::H4(heads) => heads.iter().for_each(|h| buff.push(h.clone())),
            Headers::H5(heads) => heads.iter().for_each(|h| buff.push(h.clone())),
            Headers::H6(heads) => heads.iter().for_each(|h| buff.push(h.clone())),
            _ => {}
        }
        buff
    }

    fn headers_from_tag(index: u8, headers: Vec<String>) -> Headers {
        match index {
            1 => Headers::H1(headers),
            2 => Headers::H2(headers),
            3 => Headers::H3(headers),
            4 => Headers::H4(headers),
            5 => Headers::H5(headers),
            6 => Headers::H6(headers),
            _ => Headers::Invalid,
        }
    }
}
