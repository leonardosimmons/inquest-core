use crate::cli::Cli;
use crate::service::{Request, Response};
use pin_project::pin_project;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::{event, Level};
use crate::logging::CLI;

pub(crate) struct CliService<S> {
    inner: S,
}

#[pin_project]
pub(crate) struct CliServiceFuture<F> {
    #[pin]
    future: F,
}

pub(crate) struct CliLayer;

// === Service ===

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

// === Future ===

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

// === Layer ===

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
