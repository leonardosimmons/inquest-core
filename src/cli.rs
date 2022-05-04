use std::fmt::Debug;
use std::ops::Deref;
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::{event, Level};
use crate::logging::CLI;

#[derive(StructOpt, Clone, Debug)]
pub struct HtmlParseOpts {
    /// Filter based on HTML tag
    #[structopt(short, long)]
    pub tags: Option<Vec<String>>,
    /// File paths to be probed
    #[structopt(parse(from_os_str), short, long)]
    pub paths: Option<Vec<PathBuf>>,
    /// Urls to be probed
    #[structopt(short, long)]
    pub urls: Option<Vec<String>>,
}

#[derive(StructOpt, Clone, Debug)]
pub enum HtmlOpts {
    /// Returns the meta description
    #[structopt(name = "desc")]
    Description(HtmlParseOpts),
    /// Returns the specified headers
    #[structopt(name = "headers")]
    Headers(HtmlParseOpts),
    /// Returns the specified links
    #[structopt(name = "links")]
    Links(HtmlParseOpts),
    /// Returns the title of the page
    #[structopt(name = "title")]
    PageTitle(HtmlParseOpts),
    /// Error Value
    NotSelected,
}

#[derive(StructOpt, Clone, Debug)]
pub enum CommandOpts {
    #[structopt(name = "probe")]
    /// Probes specified Html document
    Probe(HtmlOpts),
}

#[derive(StructOpt, Clone, Debug)]
pub struct Cli {
    #[structopt(subcommand)]
    /// System Command Options
    cmd: Option<CommandOpts>,
}

impl Cli {
    /// Instantiates CLI and returns command line arguments
    pub fn init() -> Cli {
        event!(target: CLI, Level::DEBUG, "cli initialized");
        Cli::from_args()
    }

    /// Returns command selected by user via the cli
    pub(crate) fn command(self) -> HtmlOpts {
        match self.cmd {
            Some(cmd) => match cmd {
                CommandOpts::Probe(opts) => opts,
            },
            None => HtmlOpts::NotSelected,
        }
    }
}

impl Deref for Cli {
    type Target = CommandOpts;

    fn deref(&self) -> &Self::Target {
        self.cmd.as_ref().unwrap()
    }
}
