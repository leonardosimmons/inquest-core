#![allow(unused)]
use crate::error::Error;
use crate::utils::Result;
use std::future::Future;
use std::task::{Context, Poll};

const APP: &str = "app";
const CLI: &str = "cli";
const SYSTEM: &str = "system";

 pub trait IntoRequest<B> {
     fn into_request(self) -> Request<B>;
 }
 pub trait IntoResponse<B> {
     fn into_response(self) -> Response<B>;
 }

#[derive(Debug)]
 pub struct Request<B> {
     body: B,
 }

#[derive(Debug)]
 pub struct Response<B> {
     body: B
 }

#[derive(Debug)]
pub struct Service<F> {
    f: F,
}

// === impl Request ===

impl<B> Request<B> {
    pub fn new(body: B) -> Request<B> {
        Self { body }
    }

    pub fn into_body(self) -> B {
        self.body
    }
}

impl<B> IntoRequest<B> for Request<B> {
    fn into_request(self) -> Request<B> {
        self
    }
}

// === impl Response ===

impl<B> Response<B> {
    pub fn new(body: B) -> Response<B> {
        Self { body }
    }

    pub fn into_body(self) -> B {
        self.body
    }
}

impl<B> IntoResponse<B> for Response<B> {
    fn into_response(self) -> Response<B> {
        self
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
