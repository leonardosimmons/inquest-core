#![allow(unused)]
use structopt::StructOpt;

pub mod parse;
use parse::ParseCommand;

pub mod html;
use html::HtmlCommand;

#[derive(StructOpt)]
#[structopt(name = "command")]
pub enum SystemCommand {
    #[structopt(name = "parse")]
    Parse,
}
