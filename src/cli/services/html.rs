use std::fmt::{Debug, Display};
use std::future::Future;
use std::pin::Pin;
use crate::cli::{CommandOpts, HtmlOpts};
use crate::error::Error;
use crate::service::{IntoRequest, Request, Response};
use futures::future;
use pin_project::pin_project;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::{event, Level};
use crate::data::Json;
use crate::logging::CLI;

pub(crate) struct HtmlOptsService<S> {
    inner: S
}

#[pin_project]
pub(crate) struct HtmlOptsServiceFuture<F> {
    #[pin]
    future: F,
}

pub(crate) struct HtmlOptsLayer;

// === Service ===

impl<S> HtmlOptsService<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<Request<Json>> for HtmlOptsService<S>
where
    S: Service<Request<Json>, Response = Response<Json>> + Send + 'static,
    S::Error: Debug + Display,
    S::Future: Send + 'static
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = HtmlOptsServiceFuture<S::Future>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Json>) -> Self::Future {
        let opts: HtmlOpts = req.into_body().data();
        match opts {
            HtmlOpts::Links(opts) => {
                let json = Json::new(opts);
                HtmlOptsServiceFuture {
                    future: self.inner.call(json.into_request())
                }
            }
            _ => panic!("unimplemented")
        }
    }
}

impl<F, E> Future for HtmlOptsServiceFuture<F>
where
    F: Future<Output = Result<Response<Json>, E>>,
    E: Debug + Display
{
    type Output = Result<Response<Json>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        event!(target: CLI, Level::TRACE, "polling html inner service(s)...");
        let res: F::Output = match this.future.poll(cx) {
            Poll::Ready(res) => res,
            Poll::Pending => return Poll::Pending,
        };

        match &res {
            Ok(_) => event!(target: CLI, Level::TRACE, "html options reviewed"),
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

impl HtmlOptsLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for HtmlOptsLayer {
    type Service = HtmlOptsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HtmlOptsService::new(inner)
    }
}
