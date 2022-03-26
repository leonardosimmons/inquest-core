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

    pub async fn document(&self) -> Document {
        Document::from_read(&*self.html.clone()).unwrap_or_else(|_| Document::from(""))
    }

    pub fn text(&self) -> String {
        str::from_utf8(&*self.html).unwrap().to_string()
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
    pub async fn all_links(&self) -> Vec<String> {
        Parse::link(self.parse.document().await, Name("a"))
    }

    pub async fn links<P: Predicate>(&self, predicate: P) -> Vec<String> {
        Parse::link(self.parse.document().await, predicate)
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
