use crate::error::{Error, ErrorKind};
use crate::html::{Headers, HtmlAttribute, HtmlDocument, HtmlParser, HtmlTag};
use crate::utils::Result;
use async_trait::async_trait;
use select::predicate::{Name, Predicate};
use std::path::PathBuf;

#[async_trait]
pub trait FromPath
where
    Self: Sized,
{
    async fn from(&mut self, path: &str, capacity: usize) -> Result<Self>;
}

#[async_trait]
pub trait FromUrl
where
    Self: Sized,
{
    async fn from(&mut self, url: &str) -> Result<Self>;
}

pub trait Parser {}

pub struct Default;

pub struct Utils;
impl Utils {
    fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches(":")
            .trim_start_matches("//")
            .trim_end_matches('/')
    }

    fn path_to_string(path: PathBuf) -> String {
        path.to_str().unwrap_or_default().to_string()
    }
}

pub struct Parse<T> {
    parse: T,
}

impl<T> Parse<T> {
    pub fn default() -> Parse<Default> {
        Parse { parse: Default }
    }

    pub fn new(kind: T) -> Parse<T> {
        Parse { parse: kind }
    }
}

impl<T> Parser for Parse<T> {}

#[async_trait]
impl<T> FromPath for Parse<T>
where
    T: FromPath + Send,
{
    async fn from(&mut self, path: &str, capacity: usize) -> Result<Self> {
        Ok(Self {
            parse: self.parse.from(path, capacity).await?,
        })
    }
}

#[async_trait]
impl<T> FromUrl for Parse<T>
where
    T: FromUrl + Send,
{
    async fn from(&mut self, url: &str) -> Result<Self> {
        Ok(Self {
            parse: self.parse.from(url).await?,
        })
    }
}

impl<T> Parse<T>
where
    T: HtmlDocument,
{
    pub fn all_headers(&self, mut buff: Vec<Headers>) -> Result<Vec<Headers>> {
        let mut count = 1;
        loop {
            match count {
                1 => buff.push(self.header(HtmlTag::H1)?),
                2 => buff.push(self.header(HtmlTag::H2)?),
                3 => buff.push(self.header(HtmlTag::H3)?),
                4 => buff.push(self.header(HtmlTag::H4)?),
                5 => buff.push(self.header(HtmlTag::H5)?),
                6 => buff.push(self.header(HtmlTag::H6)?),
                _ => break,
            };
            count = count + 1;
        }
        Ok(buff)
    }

    pub fn all_links(&self) -> Result<Vec<String>> {
        self.links(Name(HtmlAttribute::A.into()))
    }
}

impl<T> HtmlParser for Parse<T>
where
    T: HtmlDocument,
{
    fn descriptions(&self) -> Result<Vec<String>> {
        if let Ok(doc) = self.parse.document() {
            Ok(doc
                .find(Name(HtmlTag::Meta.into()))
                .filter_map(|n| match n.attr(HtmlAttribute::Name.into()) {
                    Some(name) => {
                        if name.contains("description") {
                            Some(n.attr(HtmlAttribute::Content.into()).unwrap())
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .map(|c| c.to_string())
                .collect())
        } else {
            Err(Error::from(ErrorKind::Document))
        }
    }

    fn header(&self, header: HtmlTag) -> Result<Headers> {
        if let Ok(doc) = self.parse.document() {
            Ok(Headers::new(
                doc.find(Name(&header.to_string()[..]))
                    .filter_map(|n| Some(n.text()))
                    .collect(),
                header.into(),
            ))
        } else {
            Err(Error::from(ErrorKind::Document))
        }
    }

    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        if let Ok(doc) = self.parse.document() {
            Ok(doc
                .find(predicate)
                .filter_map(|n| {
                    if let Some(link) = n.attr(HtmlAttribute::Href.into()) {
                        Some(Parse::<Utils>::fix_link(link))
                    } else {
                        None
                    }
                })
                .map(|x| x.to_string())
                .collect())
        } else {
            Err(Error::from(ErrorKind::Document))
        }
    }

    fn page_title(&self) -> Result<Vec<String>> {
        if let Ok(doc) = self.parse.document() {
            Ok(doc
                .find(Name(HtmlTag::Title.into()))
                .map(|t| t.text())
                .collect())
        } else {
            Err(Error::from(ErrorKind::Document))
        }
    }
}

impl Parse<Utils> {
    pub fn fix_link(link: &str) -> &str {
        Utils::fix_link(link)
    }

    pub fn path_to_string(path: PathBuf) -> String {
        Utils::path_to_string(path)
    }
}
