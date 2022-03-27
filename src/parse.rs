use select::document::Document;
use select::predicate::{Name, Predicate};

use crate::html::{Headers, Html, HtmlTag};

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
    fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches("://")
            .trim_end_matches('/')
    }

    fn header(tag: u8, headers: Vec<String>) -> Headers {
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

    fn header_tag(document: &Document, header: String) -> Vec<String> {
        document
            .find(Name(&header[..]))
            .filter_map(|n| Some(n.text()))
            .collect::<Vec<String>>()
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

    pub async fn all_links(&self) -> Result<Vec<String>, ParseError> {
        match self.parse.document().await {
            Ok(doc) => Ok(Parse::link(&doc, Name("a"))),
            Err(err) => Err(err),
        }
    }

    pub async fn all_headers(&self) -> Result<Vec<Headers>, ParseError> {
        match self.parse.document().await {
            Ok(doc) => {
                let mut buffer: Vec<_> = vec![];
                for i in 1..=6 {
                    buffer.push(Parse::header(i, Parse::header_tag(&doc, format!("h{}", i))))
                }
                Ok(buffer)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn headers(&self, header: HtmlTag) -> Result<Headers, ParseError> {
        match self.parse.document().await {
            Ok(doc) => match header {
                HtmlTag::H1 => Ok(Parse::header(1, Parse::header_tag(&doc, format!("h{}", 1)))),
                HtmlTag::H2 => Ok(Parse::header(2, Parse::header_tag(&doc, format!("h{}", 2)))),
                HtmlTag::H3 => Ok(Parse::header(3, Parse::header_tag(&doc, format!("h{}", 3)))),
                HtmlTag::H4 => Ok(Parse::header(4, Parse::header_tag(&doc, format!("h{}", 4)))),
                HtmlTag::H5 => Ok(Parse::header(5, Parse::header_tag(&doc, format!("h{}", 5)))),
                HtmlTag::H6 => Ok(Parse::header(6, Parse::header_tag(&doc, format!("h{}", 6)))),
                _ => Err(ParseError::Other(String::from("not a valid html `h` tag"))),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>, ParseError> {
        match self.parse.document().await {
            Ok(doc) => Ok(Parse::link(&doc, predicate)),
            Err(err) => Err(err),
        }
    }
}
