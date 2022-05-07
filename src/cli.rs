use crate::error::Error;
use crate::logging::CLI;
use crate::service::{Request, Response};
use futures::future;
use pin_project::pin_project;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use structopt::StructOpt;
use tower::Service;
use tracing::{event, Level};

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
    /// Probes specified Html document
    #[structopt(name = "probe")]
    Probe(HtmlOpts),
}

#[derive(StructOpt, Clone, Debug)]
pub struct Cli {
    /// System Command Options
    #[structopt(subcommand)]
    cmd: Option<CommandOpts>,
}

pub struct CliService<S> {
    inner: S,
}

#[pin_project]
pub struct CliServiceFuture<F> {
    #[pin]
    future: F,
}

pub struct HtmlOptsService;

#[pin_project]
pub struct HtmlOptsServiceFuture<F> {
    #[pin]
    future: F
}

// === impl Cli ===

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

// === impl CliService ===

impl<S, B> CliService<S>
where
    S: Service<Request<Cli>, Response = Response<B>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: 'static + Send,
{
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<Cli>> for CliService<S>
where
    S: Service<Request<Cli>, Response = Response<B>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: 'static + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = CliServiceFuture<S::Future>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        event!(target: CLI, Level::TRACE, "polling cli service...");
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Cli>) -> Self::Future {
        CliServiceFuture {
            future: self.inner.call(req),
        }
    }
}

impl<F, B, E> Future for CliServiceFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
    E: Debug + Display,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        event!(target: CLI, Level::TRACE, "polling cli inner service(s)...");
        let res: F::Output = match this.future.poll(cx) {
            Poll::Ready(res) => res,
            Poll::Pending => return Poll::Pending,
        };

        match &res {
            Ok(_) => event!(target: CLI, Level::DEBUG, "cli service complete"),
            Err(err) => event!(
                target: CLI,
                Level::ERROR,
                "error processing cli service; {}",
                err
            ),
        };
        Poll::Ready(res)
    }
}

// === impl HtmlOptsService ===

impl HtmlOptsService {
    pub fn new() -> Self {
        Self
    }
}

impl Service<Request<Cli>> for HtmlOptsService {
    type Response = Response<HtmlOpts>;
    type Error = Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Cli>) -> Self::Future {
        let cli = req.body().clone();
        let opts = cli.command();
        future::ready(Ok(Response::new(opts)))
    }
}
