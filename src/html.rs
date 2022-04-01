use std::str;

use bytes::Bytes;
use select::document::Document;

use crate::parse::ParseError;

pub enum HtmlTag {
    A,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Body,
    Footer,
    Main
}

pub enum Headers {
    H1(Vec<String>),
    H2(Vec<String>),
    H3(Vec<String>),
    H4(Vec<String>),
    H5(Vec<String>),
    H6(Vec<String>),
    Invalid
}

pub struct Html {
    pub(crate) html: Bytes,
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
