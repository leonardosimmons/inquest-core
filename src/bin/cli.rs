use inquest::cli::{Cli, HtmlOpts};
use inquest::probe::Probe;

#[tokio::main]
async fn main() {
    let cli = Cli::init();

    match cli.command() {
        HtmlOpts::Links(opts) => match opts.tags {
            Some(_tags) => {
                // implement -t flag -> filters links based on provided predicate(s)
            }
            None => {
                let links = Probe::all_links(opts.paths.unwrap_or_default())
                    .await
                    .unwrap();
                links.iter().for_each(|link| println!("{}", link));
            }
        },
        _ => eprintln!("unimplemented"),
    }
}
