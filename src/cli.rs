use crate::error::Error;
use crate::logging::CLI;
use crate::service::{IntoRequest, Request, Response};
use futures::future;
use pin_project::pin_project;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use structopt::StructOpt;
use tower::util::BoxService;
use tower::{Layer, Service, ServiceBuilder};
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
    /// Error Value
    NotSelected,
}

// === Cli ===

#[derive(StructOpt, Clone, Debug)]
pub struct Cli {
    /// System Command Options
    #[structopt(subcommand)]
    cmd: Option<CommandOpts>,
}

struct CliService<S> {
    inner: S,
}

#[pin_project]
struct CliServiceFuture<F> {
    #[pin]
    future: F,
}

struct CliLayer;

// === Command ===

struct CliCommand<S> {
    inner: S,
}

#[pin_project]
struct CliCommandFuture<F> {
    #[pin]
    future: F,
}

struct CommandLayer;

// === Html Opts ===

struct HtmlOptsService;

#[pin_project]
struct HtmlOptsServiceFuture<F> {
    #[pin]
    future: F,
}

struct HtmlOptsLayer;

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

    /// Root Cli Service
    pub fn service() -> BoxService<Request<Cli>, Response<CommandOpts>, Error> {
        let srv = ServiceBuilder::new()
            .layer(CliLayer::new())
            .layer(CommandLayer::new())
            .service_fn(|req: Request<CommandOpts>| async move {
                let req = req.into_body();
                let res = Response::new(req);
                Ok::<_, Error>(res)
            });
        BoxService::new(srv)
    }
}

impl IntoRequest<Cli> for Cli {
    fn into_request(self) -> Request<Cli> {
        Request::new(self)
    }
}

// === impl CliService ===

impl<S, B> CliService<S>
where
    S: Service<Request<Cli>, Response = Response<B>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: 'static + Send,
{
    fn new(inner: S) -> Self {
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
            Ok(_) => event!(target: CLI, Level::TRACE, "cli service complete"),
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

impl CliLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Layer<S> for CliLayer
where
    S: Service<Request<Cli>, Response = Response<B>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: Send + 'static,
{
    type Service = CliService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CliService::new(inner)
    }
}

// === impl CliCommand ===

impl<S> CliCommand<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<Cli>> for CliCommand<S>
where
    S: Service<Request<CommandOpts>, Response = Response<B>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: 'static + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = CliCommandFuture<S::Future>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Cli>) -> Self::Future {
        let cli = req.into_body();
        let opts = cli.command();
        CliCommandFuture {
            future: self.inner.call(Request::new(opts)),
        }
    }
}

impl<F, B, E> Future for CliCommandFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
    E: Debug + Display,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        event!(
            target: CLI,
            Level::TRACE,
            "polling cli command service(s)..."
        );
        let res: F::Output = match this.future.poll(cx) {
            Poll::Ready(res) => res,
            Poll::Pending => return Poll::Pending,
        };

        match &res {
            Ok(_) => event!(target: CLI, Level::TRACE, "cli command extracted"),
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

impl CommandLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Layer<S> for CommandLayer
where
    S: Service<Request<CommandOpts>, Response = Response<B>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: 'static + Send,
{
    type Service = CliCommand<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CliCommand::new(inner)
    }
}

// === impl HtmlOpts ===

impl HtmlOptsService {
    fn new() -> Self {
        Self
    }
}

impl Service<Request<CommandOpts>> for HtmlOptsService {
    type Response = Response<HtmlOpts>;
    type Error = Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<CommandOpts>) -> Self::Future {
        let opts = match req.into_body() {
            CommandOpts::Probe(opts) => opts,
            _ => HtmlOpts::NotSelected,
        };
        future::ready(Ok(Response::new(opts)))
    }
}

impl HtmlOptsLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for HtmlOptsLayer {
    type Service = HtmlOptsService;

    fn layer(&self, _inner: S) -> Self::Service {
        HtmlOptsService::new()
    }
}
