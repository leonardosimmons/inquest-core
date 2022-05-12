use crate::cli::{Cli, CommandOpts};
use crate::service::{Request, Response};
use pin_project::pin_project;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::{event, Level};
use crate::data::Json;
use crate::logging::CLI;

pub(crate) struct CommandOptsService<S> {
    inner: S,
}

#[pin_project]
pub(crate) struct CommandOptsFuture<F> {
    #[pin]
    future: F,
}

pub(crate) struct CommandLayer;

// === Service ===

impl<S> CommandOptsService<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<Request<Json>> for CommandOptsService<S>
where
    S: Service<Request<Json>, Response = Response<Json>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: 'static + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = CommandOptsFuture<S::Future>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Json>) -> Self::Future {
        let cli: Cli = Json::deserialize(&req.into_body().bytes());
        let opts = cli.command();
        // TODO: utilize tower::steer to match between command options
        let json = Json::new(opts);
        CommandOptsFuture {
            future: self.inner.call(Request::new(json)),
        }
    }
}

// === Future ===

impl<F, E> Future for CommandOptsFuture<F>
where
    F: Future<Output = Result<Response<Json>, E>>,
    E: Debug + Display,
{
    type Output = Result<Response<Json>, E>;

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

// === Layer ===

impl CommandLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for CommandLayer
where
    S: Service<Request<Json>, Response = Response<Json>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: 'static + Send,
{
    type Service = CommandOptsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CommandOptsService::new(inner)
    }
}
