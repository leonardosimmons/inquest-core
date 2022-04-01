use clap::Parser;
use std::path::PathBuf;
use crate::html::Html;
use crate::parse::Parse;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    pub search: String,

    #[clap(parse(from_os_str), default_value = "")]
    pub path: PathBuf,

    #[clap(short, long, default_value = "")]
    pub url: String,
}
impl Cli {
    pub fn new() -> Cli {
        let cli = Cli::parse();
        Cli {
            search: cli.search,
            path: cli.path,
            url: cli.url
        }
    }
}
