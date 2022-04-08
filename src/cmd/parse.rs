use structopt::StructOpt;

use crate::cmd::html::HtmlCommand;
use crate::utils::{Responder, Result};

#[derive(StructOpt)]
pub enum ParseCommand{
    Html {
        cmd: Box<HtmlCommand>,
        // resp: Responder<Result<T>>
    },
    Unknown
}
