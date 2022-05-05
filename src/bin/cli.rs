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
    System::<_, Initialized>::init(cli);
}

// async fn handle<S>(req: Request<S>) -> Result<Response, Error> {
//     tokio::time::sleep(Duration::from_secs(1)).await;
//     Ok::<_, Error>(Response::new(req.cli()))
// }
// let service = Service::create(move |req: Request| async move {
//     tokio::time::sleep(Duration::from_secs(1)).await;
//     Ok::<_, Error>(Response::new(req.cli()))
// });

// let service = Service::create(move |req: Request| async move {
// let middleware = ServiceBuilder::new()
// .layer(LoggingLayer::new())
// .service(Service::create(handle));
// Ok::<_, Error>(Response::new())
// });
