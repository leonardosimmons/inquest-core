use env_logger::Env;
use inquest::cli::{Cli, HtmlOpts};
use inquest::probe::Probe;

const LOGGING_FILTER: &str = "client=trace";

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or(LOGGING_FILTER));

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
