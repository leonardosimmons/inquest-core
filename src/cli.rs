use std::ops::Deref;
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::{event, Level};

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
    /// Error Value
    NotSelected
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
        event!(Level::DEBUG, "cli initialized");
        Cli::from_args()
    }

    pub fn command(self) -> HtmlOpts {
        match self.cmd {
            Some(cmd) => match cmd {
                CommandOpts::Probe(opts) => opts,
            }
            None => HtmlOpts::NotSelected
        }
    }
}
impl Deref for Cli {
    type Target = CommandOpts;

    fn deref(&self) -> &Self::Target {
        self.cmd.as_ref().unwrap()
    }
}
