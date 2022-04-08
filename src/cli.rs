use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Inquest", about = "A SEO Utility Tool")]
pub struct Cli {
    pub search: String,

    #[structopt(parse(from_os_str), short, long, default_value = "")]
    pub path: PathBuf,

    #[structopt(short, long, default_value = "")]
    pub url: String,
}
impl Cli {
    pub fn new() -> Cli {
        let cli = Cli::from_args();
        Cli {
            search: cli.search,
            path: cli.path,
            url: cli.url
        }
    }
}
