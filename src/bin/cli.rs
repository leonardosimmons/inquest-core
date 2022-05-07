use inquest::cli::{Cli, CliService, HtmlOptsService};
use inquest::service::Request;
use inquest::system::System;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .init();

    let cli = Cli::init();

    let app = CliService::new(HtmlOptsService::new());
    let req = Request::new(cli);

    System::run(app, req).await;
}
