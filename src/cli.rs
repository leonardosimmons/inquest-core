#![allow(unused)]
use std::path::PathBuf;
use structopt::StructOpt;

use crate::cmd::parse::ParseCommand;

#[derive(StructOpt)]
#[structopt(name = "Inquest", about = "A SEO Utility Tool")]
pub struct Cli {
    #[structopt(subcommand)]
    parser: ParseCommand,

    pub search: String,

    #[structopt(parse(from_os_str), short, long, default_value = "")]
    pub path: PathBuf,

    #[structopt(short, long, default_value = "")]
    pub url: String,
}
impl Cli {
    pub fn new() -> Cli {
        let cli = Cli::from_args();
        Cli {
            parser: ParseCommand::Unknown,
            search: cli.search,
            path: cli.path,
            url: cli.url
        }
    }
}
