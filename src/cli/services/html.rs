use crate::cli::{CommandOpts, HtmlOpts};
use crate::error::Error;
use crate::service::{Request, Response};
use futures::future;
use pin_project::pin_project;
use std::task::{Context, Poll};
use tower::{Layer, Service};

pub(crate) struct HtmlOptsService;

#[pin_project]
pub(crate) struct HtmlOptsServiceFuture<F> {
    #[pin]
    future: F,
}

pub(crate) struct HtmlOptsLayer;

// === Service ===

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

// === Layer ===

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
