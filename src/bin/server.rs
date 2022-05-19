#![allow(unused)]
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tracing::Level;

const IP_ADDRESS: [u8; 4] = [127, 0, 0, 1];
const LOGGING_FILTER: &str = "server=trace";
const PORT: u16 = 3000;
const TEST_HANDLE_DURATION: u64 = 2;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((IP_ADDRESS, PORT));

    let service = make_service_fn(|_conn| async {
        let svc = ServiceBuilder::new().service(service_fn(test_handle));
        Ok::<_, Infallible>(svc)
    });

    let server = Server::bind(&addr).serve(service);

    tracing::info!("Listening on port: {}", PORT);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e)
    }
}

async fn test_handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    tokio::time::sleep(Duration::from_secs(TEST_HANDLE_DURATION)).await;
    tracing::trace!("test_handle fired");
    Ok(Response::new(Body::from("Hello Portland")))
}
