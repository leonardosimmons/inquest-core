#![allow(unused)]
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

use bytes::Bytes;
use select::document::Document;
use select::predicate::{Name, Predicate};

use crate::error::{Error, ErrorKind};
use crate::html::{Headers, Html, HtmlController, RawHtml, HtmlParser, HtmlTag};
use crate::utils::{Result, Unknown};

pub trait Parser {
    fn path_to_string(paths: PathBuf) -> String;
}

pub struct Base;

pub struct Parse<T> {
    parse: T,
}

impl<T> Parse<T> {
    pub fn new(kind: T) -> Parse<T> {
        Parse { parse: kind }
    }
    
    pub fn set(mut self, kind: T) -> Self { 
        self.parse = kind;
        self
    }
}

impl<T> HtmlController for Parse<T>
    where
        T: HtmlController
{
    fn descriptions(&self) -> Result<Vec<String>> {
        self.parse.descriptions()
    }

    fn headers(&self, header: HtmlTag) -> Result<Headers> {
        self.parse.headers(header)
    }

    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        self.parse.links(predicate)
    }

    fn page_title(&self) -> Result<Vec<String>> {
        self.parse.page_title()
    }
}

impl<T> Parser for Parse<T> {
    fn path_to_string(paths: PathBuf) -> String {
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

    pub async fn all_headers(&self, mut buff: Vec<Headers>) -> Result<Vec<Headers>> {
        use HtmlTag::*;

        let mut count = 1;
        loop {
            match count {
                1 => buff.push(self.headers(H1).unwrap_or(Headers::H1(Vec::new()))),
                2 => buff.push(self.headers(H2).unwrap_or(Headers::H2(Vec::new()))),
                3 => buff.push(self.headers(H3).unwrap_or(Headers::H3(Vec::new()))),
                4 => buff.push(self.headers(H4).unwrap_or(Headers::H4(Vec::new()))),
                5 => buff.push(self.headers(H5).unwrap_or(Headers::H5(Vec::new()))),
                6 => buff.push(self.headers(H6).unwrap_or(Headers::H6(Vec::new()))),
                _ => break,
            };
            count = count + 1;
        }
        Ok(buff)
    }

    pub async fn all_links(&self) -> Result<Vec<String>> {
        self.links(Name("a"))
    }
}

impl HtmlParser for Parse<Html> {
    fn bytes(&self) -> Bytes {
        Arc::clone(&self.parse.html).lock().unwrap().clone()
    }

    fn document(&self) -> Result<Document> {
        if let Ok(doc) = self.parse.document() {
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

impl Default for Parse<Html> {
    fn default() -> Self {
        Parse {
            parse: Html::new("")
        }
    }
}