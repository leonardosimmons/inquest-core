#![allow(unused)]
use std::path::PathBuf;
use scraper::Html;
use inquest::cli::{Cli, HtmlOpts};
use inquest::probe::{FromFile, ProbeBuilder};

#[tokio::main]
async fn main() {
    let opts = Cli::init();
}
