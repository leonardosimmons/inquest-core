#![allow(unused)]
use crate::error::Error;
use crate::utils::Result;
use std::future::Future;
use std::task::{Context, Poll};

const APP: &str = "app";
const CLI: &str = "cli";
const SYSTEM: &str = "system";

#[derive(Debug)]
pub struct Request<Body> {
    body: Body
}

#[derive(Debug)]
pub struct Response {
    message: String,
}

#[derive(Debug)]
pub struct Service<F> {
    f: F,
}

pub struct Factory<F> {
    f: Vec<F>
}

// === impl Request ===

impl<Body> Request<Body> {
    pub fn new(body: Body) -> Request<Body> {
        Request { body }
    }
}

// === impl Response ===

impl Response {
    pub fn new(message: impl ToString) -> Response {
        Response {
            message: message.to_string(),
        }
    }
}

// === impl Service ===

impl<F> Service<F> {
    pub fn create<Req, Res, Fut>(f: F) -> Service<F>
    where
        F: FnMut(Req) -> Fut,
        Fut: Future<Output = Result<Res>>,
    {
        Service { f }
    }
}

impl<F, Req, Res, Fut> tower::Service<Req> for Service<F>
where
    F: FnMut(Req) -> Fut,
    Fut: Future<Output = Result<Res>>,
{
    type Response = Res;
    type Error = Error;
    type Future = Fut;

    fn poll_ready(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<std::prelude::rust_2015::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Req) -> Self::Future {
        (self.f)(req)
    }
}

// === impl Factory ===