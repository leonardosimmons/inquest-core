use std::str;

use bytes::Bytes;
use select::document::Document;
use select::predicate::{Name, Predicate};

pub struct Html {
    html: Bytes,
}
impl Html {
    pub fn new(html: String) -> Html {
        Html { html: html.into() }
    }

    pub async fn from_url(url: &str) -> Result<Html, ParseError> {
        match reqwest::get(url).await {
            Ok(resp) => {
                if let Ok(text) = resp.text().await {
                    Ok(Html::new(text))
                } else {
                    Err(ParseError::Failed(String::from(
                        "failed to parse html from response",
                    )))
                }
            }
            Err(err) => Err(ParseError::Other(format!(
                "unable to retrieve url {}: {}",
                url,
                err.to_string()
            ))),
        }
    }

    pub fn bytes(&self) -> Bytes {
        self.html.clone()
    }

    pub async fn document(&self) -> Result<Document, ParseError> {
        if let Ok(document) = Document::from_read(&*self.html) {
            Ok(document)
        } else {
            Err(ParseError::Failed(String::from(
                "unable to parse html into document",
            )))
        }
    }

    pub fn text(&self) -> Result<String, ParseError> {
        if let Ok(str) = str::from_utf8(&*self.html) {
            Ok(String::from(str))
        } else {
            Err(ParseError::Failed(String::from(
                "unable to parse text from html",
            )))
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    EndOfStream,
    Failed(String),
    Other(String),
}

pub struct Parse<P> {
    parse: P,
}
impl<P> Parse<P> {
    pub fn new(kind: P) -> Parse<P> {
        Parse { parse: kind }
    }
}

impl Parse<Html> {
    pub async fn all_links(&self) -> Option<Vec<String>> {
        if let Ok(document) = self.parse.document().await {
            Some(Parse::link(document, Name("a")))
        } else {
            None
        }
    }

    pub async fn links<P: Predicate>(&self, predicate: P) -> Option<Vec<String>> {
        if let Ok(document) = self.parse.document().await {
            Some(Parse::link(document, predicate))
        } else {
            None
        }
    }
}
impl LinkController for Parse<Html> {
    fn link<P: Predicate>(document: Document, predicate: P) -> Vec<String> {
        document
            .find(predicate)
            .filter_map(|n| {
                if let Some(link) = n.attr("href") {
                    Some(Parse::fix_link(link))
                } else {
                    None
                }
            })
            .map(|link| link.to_string())
            .collect()
    }

    fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches("://")
            .trim_end_matches('/')
    }
}

pub trait LinkController {
    fn link<P: Predicate>(document: Document, predicate: P) -> Vec<String>;
    fn fix_link(link: &str) -> &str;
}
