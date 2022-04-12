#![allow(unused)]

use std::borrow::Borrow;
use std::path::PathBuf;

use structopt::StructOpt;
use crate::error::{Error, ErrorKind};

use crate::utils::Result;

#[derive(StructOpt, Clone, Debug)]
pub struct HtmlParseOpts {
    #[structopt(short, long)]
    /// Filter based on HTML tag
    pub tags: Option<Vec<String>>,

    #[structopt(parse(from_os_str), short, long)]
    /// File paths to be probed
    pub paths: Option<Vec<PathBuf>>,

    #[structopt(short, long)]
    /// Urls to be probed
    pub urls: Option<Vec<String>>,
}

#[derive(StructOpt, Clone, Debug)]
pub enum HtmlOpts {
    #[structopt(name = "desc")]
    /// Returns the meta description
    Description(HtmlParseOpts),
    #[structopt(name = "headers")]
    /// Returns the specified headers
    Headers(HtmlParseOpts),
    #[structopt(name = "links")]
    /// Returns the specified links
    Links(HtmlParseOpts),
    #[structopt(name = "title")]
    /// Returns the title of the page
    PageTitle(HtmlParseOpts),
}

#[derive(StructOpt, Clone, Debug)]
pub enum CommandOpts {
    #[structopt(name = "probe")]
    /// Probes specified Html document
    Probe(HtmlOpts),
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(subcommand)]
    /// System Command Options
    cmd: Option<CommandOpts>,
}


impl Cli {
    pub fn init() -> Cli {
        Cli::from_args()
    }

    fn opts(&self) -> CommandOpts  {
        self.cmd.clone().unwrap()
    }

    pub fn command(&self) -> HtmlOpts {
        match self.opts() {
            CommandOpts::Probe(opts) => opts,
        }
    }
}
