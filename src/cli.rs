#![allow(unused)]
use crate::error::Error;
use crate::logging::CLI;
use futures::future::{self, Future};
use pin_project::pin_project;
use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use hyper::Body;
use structopt::StructOpt;
use tower::Service;
use tracing::{event, trace, Level};
use crate::service::{Request, Response};

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

impl Service<Cli> for Cli
{
    type Response = Response<Cli, HtmlOpts>;
    type Error = Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Cli) -> Self::Future {
        let cli = req.clone();
        let command = req.command();
        future::ready(Ok(Response::new(cli, command)))
    }
}

impl Service<Request<Cli, HtmlOpts>> for HtmlOpts {
    type Response = Option<Vec<String>>;
    type Error = Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Cli, HtmlOpts>) -> Self::Future {
        future::ready(Ok(Some(vec!["Test".to_string(), "Vec".to_string()])))
    }
}
