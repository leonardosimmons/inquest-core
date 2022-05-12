#![allow(unused)]
use inquest::cli::Cli;
use inquest::system::System;
use tracing::Level;
use tracing_subscriber;
use inquest::data::Json;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .init();

    let cli = Cli::init();
    let srv = Cli::service();

    let json = Json::new(cli);

    System::bind(srv).run(json).await;
}
