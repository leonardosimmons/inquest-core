#![allow(unused)]
use crate::parse::ParseCommand;

pub enum Command<T> {
    Parse(ParseCommand<T>),
    Probe,
    Unknown
}
