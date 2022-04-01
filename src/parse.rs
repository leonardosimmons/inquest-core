use select::document::Document;
use select::predicate::{Name, Predicate};
use std::sync::{Arc, Mutex};

use crate::html::{Headers, Html, HtmlTag};

#[derive(Debug)]
pub enum ParseError {
    EndOfStream,
    Failed(String),
    Other(String),
}

pub struct Parse<P> {
    parse: Arc<Mutex<P>>,
}
impl<P> Parse<P> {
    pub fn new(kind: P) -> Parse<P> {
        Parse {
            parse: Arc::new(Mutex::new(kind)),
        }
    }

    pub fn from(html: String) -> Parse<Html> {
        Parse {
            parse: Arc::new(Mutex::new(Html { html: html.into() })),
        }
    }

    pub async fn from_url(html: &str) -> Parse<Html> {
        Parse {
            parse: Arc::new(Mutex::new(Html::from_url(&html[..]).await.unwrap())),
        }
    }
}

impl Parse<Html> {
    fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches("://")
            .trim_end_matches('/')
    }

    fn header_from_tag(tag: u8, headers: Vec<String>) -> Headers {
        match tag {
            1 => Headers::H1(headers),
            2 => Headers::H2(headers),
            3 => Headers::H3(headers),
            4 => Headers::H4(headers),
            5 => Headers::H5(headers),
            6 => Headers::H6(headers),
            _ => Headers::Invalid,
        }
    }

    pub fn headers(headers: &Headers) -> Vec<String> {
        let mut buffer = vec![];
        match headers {
            Headers::H1(heads) => heads.iter().for_each(|h| buffer.push(h.clone())),
            Headers::H2(heads) => heads.iter().for_each(|h| buffer.push(h.clone())),
            Headers::H3(heads) => heads.iter().for_each(|h| buffer.push(h.clone())),
            Headers::H4(heads) => heads.iter().for_each(|h| buffer.push(h.clone())),
            Headers::H5(heads) => heads.iter().for_each(|h| buffer.push(h.clone())),
            Headers::H6(heads) => heads.iter().for_each(|h| buffer.push(h.clone())),
            _ => {}
        };
        buffer
    }

    fn header_tag(document: &Document, header: String) -> Vec<String> {
        document
            .find(Name(&header[..]))
            .filter_map(|n| Some(n.text()))
            .collect()
    }

    fn link<P: Predicate>(document: &Document, predicate: P) -> Vec<String> {
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

    fn title(document: &Document) -> Vec<String> {
        document.find(Name("title")).map(|t| t.text()).collect()
    }

    pub async fn all_headers(&self) -> Result<Vec<Headers>, ParseError> {
        match self.parse.lock().unwrap().document().await {
            Ok(doc) => {
                let mut buffer: Vec<_> = vec![];
                for i in 1..=6 {
                    buffer.push(Parse::header_from_tag(
                        i,
                        Parse::header_tag(&doc, format!("h{}", i)),
                    ))
                }
                Ok(buffer)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn all_links(&self) -> Result<Vec<String>, ParseError> {
        Ok(Parse::link(
            &self.parse.lock().unwrap().document().await?,
            Name("a"),
        ))
    }

    pub async fn header(&self, header: HtmlTag) -> Result<Headers, ParseError> {
        match self.parse.lock().unwrap().document().await {
            Ok(doc) => match header {
                HtmlTag::H1 => Ok(Parse::header_from_tag(
                    1,
                    Parse::header_tag(&doc, format!("h{}", 1)),
                )),
                HtmlTag::H2 => Ok(Parse::header_from_tag(
                    2,
                    Parse::header_tag(&doc, format!("h{}", 2)),
                )),
                HtmlTag::H3 => Ok(Parse::header_from_tag(
                    3,
                    Parse::header_tag(&doc, format!("h{}", 3)),
                )),
                HtmlTag::H4 => Ok(Parse::header_from_tag(
                    4,
                    Parse::header_tag(&doc, format!("h{}", 4)),
                )),
                HtmlTag::H5 => Ok(Parse::header_from_tag(
                    5,
                    Parse::header_tag(&doc, format!("h{}", 5)),
                )),
                HtmlTag::H6 => Ok(Parse::header_from_tag(
                    6,
                    Parse::header_tag(&doc, format!("h{}", 6)),
                )),
                _ => Err(ParseError::Other(String::from("not a valid html `h` tag"))),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>, ParseError> {
        Ok(Parse::link(
            &self.parse.lock().unwrap().document().await?,
            predicate,
        ))
    }

    pub async fn page_title(&self) -> Result<Vec<String>, ParseError> {
        Ok(Parse::title(&self.parse.lock().unwrap().document().await?))
    }
}
