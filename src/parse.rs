use std::path::PathBuf;

use select::predicate::{Name, Predicate};

use crate::error::{Error, ErrorKind};
use crate::html::{Headers, Html, HtmlDocument, HtmlParser, HtmlTag};
use crate::utils::Result;

pub trait Parser {
    fn fix_link(link: &str) -> &str;
    fn path_to_string(paths: PathBuf) -> String;
}

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

impl<T> Parser for Parse<T> {
    fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches(":")
            .trim_start_matches("//")
            .trim_end_matches('/')
    }

    fn path_to_string(paths: PathBuf) -> String {
        paths.to_str().unwrap_or_default().to_string()
    }
}

impl<T> Parse<T>
where
    T: HtmlDocument
{
    pub async fn from(document: &str, buf: &[u8]) -> Result<Parse<Html>> {
        Ok(Parse {
            parse: Html::from(document, buf).await?,
        })
    }

    pub async fn from_url(html: &str) -> Result<Parse<Html>> {
        Ok(Parse::new(Html::from_url(html).await?))
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

impl<T> HtmlParser for Parse<T>
where
    T: HtmlDocument,
{
    fn descriptions(&self) -> Result<Vec<String>> {
        if let Ok(doc) = self.parse.document() {
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
        match self.parse.document() {
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
            Err(_) => Err(Error::from(ErrorKind::Parse)),
        }
    }

    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        if let Ok(doc) = self.parse.document() {
            Ok(doc
                .find(predicate)
                .filter_map(|n| {
                    if let Some(link) = n.attr("href") {
                        Some(Parse::<&str>::fix_link(link))
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
        if let Ok(doc) = self.parse.document() {
            Ok(doc.find(Name("title")).map(|t| t.text()).collect())
        } else {
            Err(Error::from(ErrorKind::Html))
        }
    }
}
