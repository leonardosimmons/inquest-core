use select::document::Document;
use select::predicate::{Name, Predicate};

use crate::html::Html;

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

    fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches("://")
            .trim_end_matches('/')
    }

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

    pub async fn links<P: Predicate>(&self, predicate: P) -> Option<Vec<String>> {
        if let Ok(document) = self.parse.document().await {
            Some(Parse::link(document, predicate))
        } else {
            None
        }
    }

    pub async fn all_titles(&self) -> Vec<String> {
        if let Ok(document) = self.parse.document().await {
            let mut buffer: Vec<_> = vec![];
            for i in 1..=6 {
                document
                    .find(Name(&format!("h{}", i)[..]))
                    .filter_map(|n| Some(n.text()))
                    .for_each(|item| buffer.push(item.trim_start_matches("").to_string()))
            }
            buffer
        } else {
            vec!["".to_string()]
        }
    }
}
