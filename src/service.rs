#![allow(unused)]
use crate::error::Error;
use crate::utils::Result;
use std::future::Future;
use std::task::{Context, Poll};

const APP: &str = "app";
const CLI: &str = "cli";
const SYSTEM: &str = "system";

 pub trait IntoRequest {
     fn into_request<S, B>(self) -> Request<S, B>;
 }
 pub trait IntoResponse {
     fn into_request<S, B>(self) -> Response<S, B>;
 }

#[derive(Debug)]
 pub struct Request<S, B> {
     inner: S,
     body: B,
 }

#[derive(Debug)]
 pub struct Response<S, B> {
     inner: S,
     body: B
 }

 impl<S, B> Request<S, B> {
     pub fn new(inner: S, body: B) -> Request<S, B> {
         Self { inner, body }
     }

     pub fn inner(&self) -> &S {
         &self.inner
     }

     pub fn body(&self) -> &B {
         &self.body
     }
 }

 impl<S, B> Response<S, B> {
     pub fn new(inner: S, body: B) -> Response<S, B> {
         Self { inner, body }
     }

     pub fn inner(&self) -> &S {
         &self.inner
     }

     pub fn body(&self) -> &B {
         &self.body
     }
 }

#[derive(Debug)]
pub struct Service<F> {
    f: F,
}

pub struct Factory<F> {
    f: Vec<F>
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