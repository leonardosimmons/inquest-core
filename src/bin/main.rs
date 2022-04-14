#![allow(unused)]
use inquest::cli::{Cli, HtmlOpts};
use inquest::probe::Probe;

#[tokio::main]
async fn main() {
    let opts = Cli::init();

    match opts.command() {
        HtmlOpts::Links(opts) => {
            println!("tags: {:?}", opts.tags.as_ref().unwrap());
            println!("path: {:?}", opts.paths.as_ref().unwrap());
        }
        _ => println!("unimplemented")
    }
}
