use inquest::cli::{Cli, HtmlOpts};
use inquest::probe::Probe;
use pretty_env_logger;

const LOGGING_FILTER: &str = "client=trace";

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder().parse_filters(LOGGING_FILTER).init();

    let cli = Cli::init();

    match cli.command() {
        HtmlOpts::Links(opts) => match opts.tags {
            Some(_tags) => {
                // implement -t flag -> filters links based on provided predicate(s)
            }
            None => {
                let probe = Probe::new().document().buffer(4096).html();
                match probe.from("tests/test.html").await {
                    Ok(probe) => {
                        if let Ok(links) = probe.all_links() {
                            links.iter().for_each(|link| println!("{}", link));
                        } else {
                            eprintln!("error: failed to retrieve descriptions");
                        }
                    }
                    Err(err) => eprintln!("error: {}", err.to_string()),
                }

            }
        },
        _ => eprintln!("unimplemented"),
    }
}
