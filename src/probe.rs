use clap::Parser;
use std::path::PathBuf;
use crate::cli::Cli;
use crate::html::Html;
use crate::parse::Parse;

pub struct Probe{}

impl Probe {
    pub async fn html(cli: &Cli, parse: Parse<Html>) -> Vec<String> {
        match &cli.search[..] {
            "links" => parse.all_links().await.unwrap(),
            "page title" => parse.page_title().await.unwrap(),
            "headers" => {
                let mut headers = vec![];
                parse
                    .all_headers()
                    .await
                    .unwrap()
                    .iter()
                    .for_each(|h| {
                        let vec = Parse::headers(h);
                        vec.iter().for_each(|i| headers.push(i.clone()))
                    });
                headers
            },
            _ => vec![],
        }
    }
}
