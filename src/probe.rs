use crate::cli::Cli;
use crate::error::{Error, ErrorKind};
use crate::html::Html;
use crate::parse::Parse;
use crate::utils::Result;

pub struct Probe {}

impl Probe {
    pub async fn html(parse: Parse<Html>, cli: &Cli) -> Result<Vec<String>> {
        match &cli.search[..] {
            "links" => Ok(parse.all_links().await?),
            "page_title" => Ok(parse.page_title().await?),
            "headers" => {
                let mut headers = Vec::new();
                parse.all_headers()
                    .await?
                    .iter()
                    .for_each(|h| {
                        let vec = Parse::headers(h);
                        vec.iter().for_each(|i| headers.push(i.clone()))
                    });
                Ok(headers)
            },
            _ => Err(Error::from(ErrorKind::InvalidSearch))
        }
    }
}
