use inquest::cli::{Cli, CliLayer, HtmlOptsLayer};
use inquest::error::Error;
use inquest::service::{Request, Response};
use inquest::system::System;
use tower::ServiceBuilder;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .init();

    let cli = Cli::init();
    let req = Request::new(cli);

    let cli = ServiceBuilder::new()
        .layer(CliLayer::new())
        .layer(HtmlOptsLayer::new())
        .service_fn(|req: Cli| Ok::<_, Error>(Response::new(req)));

    System::bind(cli).run(req).await;
}
