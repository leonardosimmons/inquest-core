#![allow(unused)]
use inquest::cli::{Cli, HtmlOpts};
use inquest::probe::{Probe, ProbeMain};

#[tokio::main]
async fn main() {
    let opts = Cli::init();

    match opts.command() {
        HtmlOpts::Links(opts) => {
            let test = ProbeMain::new().capacity(5000).document().;
            println!("tags: {:?}", opts.tags.as_ref().unwrap());
            println!("path: {:?}", opts.paths.as_ref().unwrap());
        }
        _ => println!("unimplemented")
    }
}
