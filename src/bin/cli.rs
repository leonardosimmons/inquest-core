#![allow(unused)]
use hyper::service::{make_service_fn, service_fn};
use inquest::cli::Cli;
use inquest::error::Error;
use inquest::logging::LoggingLayer;
use inquest::service::{Request, Service};
use std::convert::Infallible;
use std::time::Duration;
use tower::ServiceBuilder;
use tracing::Level;
use tracing_subscriber;
use inquest::system::{Initialized, System};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .init();

    let cli = Cli::init();
    let system = System::init(cli);
}
