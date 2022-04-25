use env_logger::Env;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;

const LOGGING_FILTER: &str = "server=trace";
const TEST_HANDLE_DURATION: u64 = 2;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or(LOGGING_FILTER));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = make_service_fn(|_conn| async {
        let svc = ServiceBuilder::new().service(service_fn(test_handle));
        Ok::<_, Infallible>(svc)
    });

    let server = Server::bind(&addr).serve(service);

    log::trace!("Server initialized");

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e)
    }
}

async fn test_handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    tokio::time::sleep(Duration::from_secs(TEST_HANDLE_DURATION)).await;
    log::trace!("test_handle fired");
    Ok(Response::new(Body::from("Hello Portland")))
}
