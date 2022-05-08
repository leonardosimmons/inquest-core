use inquest::cli::Cli;
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
    let srv = Cli::service();
    let req = Request::new(cli);

    System::bind(srv).run(req).await;
}
