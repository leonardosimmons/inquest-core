#![allow(unused)]
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use select::document::Document;
use select::predicate::Name;

pub struct Html {
    text: String,
}
impl Html {
    pub fn new(html: String) -> Html {
        Html { text: html }
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn bytes(&self) -> Bytes {
        self.text.clone().into()
    }

    pub fn set(&mut self, document: String) {
        self.text = document;
    }
}

#[derive(Debug)]
pub enum ParseError {
    EndOfStream,
    Failed(String),
    Other(String),
}

pub struct Parse {
    bytes: Bytes,
}
impl Parse {
    pub fn new(string: String) -> Parse {
        Parse {
            bytes: string.into(),
        }
    }

    pub async fn html_from_url(url: &str) -> Result<Html, ParseError> {
        match reqwest::get(url).await {
            Ok(resp) => {
                if let Ok(text) = resp.text().await {
                    Ok(Html { text })
                } else {
                    Err(ParseError::Failed(format!(
                        "failed to parse html from response"
                    )))
                }
            }
            Err(err) => Err(ParseError::Other(format!(
                "unable to retrieve url: {}",
                url
            ))),
        }
    }

    pub async fn all_links(&self) -> Vec<String> {
        Document::from_read(&*self.bytes)
            .unwrap_or_else(|_| Document::from(""))
            .find(Name("a"))
            .filter_map(|n| {
                Some(
                    n.attr("href")
                        .unwrap()
                        .trim_start_matches("http")
                        .trim_start_matches("s")
                        .trim_start_matches("://")
                )
            })
            .map(|link| link.to_string())
            .collect::<Vec<String>>()
    }
}
