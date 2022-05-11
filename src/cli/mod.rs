#![allow(unused)]
use crate::cli::services::cli::CliLayer;
use crate::cli::services::commands::CommandLayer;
use crate::error::Error;
use crate::logging::{CLI, REQUEST, RESPONSE};
use crate::service::{IntoRequest, IntoResponse, Request, Response};
use bytes::Bytes;
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::PathBuf;
use structopt::StructOpt;
use tower::util::BoxService;
use tower::ServiceBuilder;
use tracing::{event, Level};

pub mod services;

#[derive(StructOpt, Clone, Debug, Deserialize, Serialize)]
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

#[derive(StructOpt, Clone, Debug, Deserialize, Serialize)]
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

#[derive(StructOpt, Clone, Debug, Deserialize, Serialize)]
pub enum CommandOpts {
    /// Probes specified Html document
    #[structopt(name = "probe")]
    Probe(HtmlOpts),
    /// Error Value
    NotSelected,
}

// === Cli ===

#[derive(StructOpt, Clone, Debug, Deserialize, Serialize)]
pub struct Cli {
    /// System Command Options
    #[structopt(subcommand)]
    cmd: Option<CommandOpts>,
}

// === impl Cli ===

impl Cli {
    /// Instantiates CLI and returns command line arguments
    pub fn init() -> Cli {
        event!(target: CLI, Level::DEBUG, "cli initialized");
        Cli::from_args()
    }

    /// Returns command selected by user via the cli
    fn command(self) -> CommandOpts {
        match self.cmd {
            Some(cmd) => cmd,
            None => CommandOpts::NotSelected,
        }
    }

    // pub fn service() -> BoxService<Request<Cli>, Response<CommandOpts>, Error> {
    //     let srv = ServiceBuilder::new()
    //         .layer(CliLayer::new())
    //         .layer(CommandLayer::new())
    //         .service_fn(|req: Request<CommandOpts>| async move {
    //             let req = req.into_body();
    //             let res = Response::new(req);
    //             Ok::<_, Error>(res)
    //         });
    //     BoxService::new(srv)
    // }
}

impl IntoRequest<Bytes> for Cli {
    fn into_request(self) -> Request<Bytes> {
        let bytes = match serde_json::to_vec(&self) {
            Ok(vec) => Bytes::from(vec),
            Err(err) => {
                event!(
                    target: REQUEST,
                    Level::ERROR,
                    "failed to parse cli into request; {}",
                    err
                );
                Bytes::default()
            }
        };
        Request::new(bytes)
    }
}

// === impl CommandOpts ===

impl IntoResponse<Bytes> for CommandOpts {
    fn into_response(self) -> Response<Bytes> {
        let bytes = match serde_json::to_vec(&self) {
            Ok(vec) => Bytes::from(vec),
            Err(err) => {
                event!(
                    target: RESPONSE,
                    Level::ERROR,
                    "failed to parse cli into response; {}",
                    err
                );
                Bytes::default()
            }
        };
        Response::new(bytes)
    }
}
