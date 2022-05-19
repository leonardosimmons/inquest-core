use hyper::{Request, Response};
use pin_project::pin_project;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Instant;
use tower::{Layer, Service};
use tracing::{event, Level};

pub(crate) const APP: &str = "app";
pub(crate) const CLI: &str = "cli";
pub(crate) const JSON: &str = "json";
pub(crate) const LOGGER: &str = "logger";
pub(crate) const LOGGING_FUTURE: &str = "logging_future";
pub(crate) const REQUEST: &str = "request";
pub(crate) const RESPONSE: &str = "response";
pub(crate) const SYSTEM: &str = "system";

#[derive(Copy, Clone, Debug)]
pub struct Logging<S> {
    inner: S,
}

impl<S> Logging<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for Logging<S>
where
    S: Service<Request<B>, Response = Response<B>> + Send + 'static,
    B: 'static + Send,
    S::Future: 'static + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LoggingFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let start = Instant::now();
        event!(
            target: LOGGER,
            Level::DEBUG,
            "started processing request; start={:?}",
            start
        );
        LoggingFuture {
            future: self.inner.call(req),
            start,
        }
    }
}

#[pin_project]
pub struct LoggingFuture<F> {
    #[pin]
    future: F,
    start: Instant,
}

impl<F, Res, E> Future for LoggingFuture<F>
where
    F: Future<Output = Result<Res, E>>,
{
    type Output = Result<Res, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        event!(
            target: LOGGING_FUTURE,
            Level::TRACE,
            "polling future; time={:?}",
            self.start.to_owned()
        );

        let this = self.project();
        let res: F::Output = match this.future.poll(cx) {
            Poll::Ready(res) => res,
            Poll::Pending => return Poll::Pending,
        };
        let duration = this.start.elapsed();
        match &res {
            Ok(_res) => event!(
                target: LOGGING_FUTURE,
                Level::DEBUG,
                "finished processing request; time elapsed={:?}",
                duration
            ),
            Err(_err) => event!(
                target: LOGGING_FUTURE,
                Level::ERROR,
                "there was an error processing your request",
            ),
        }
        Poll::Ready(res)
    }
}

pub struct LoggingLayer;

impl LoggingLayer {
    pub fn new() -> Self { Self }
}

impl<S> Layer<S> for LoggingLayer {
    type Service = Logging<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Logging::new(inner)
    }
}