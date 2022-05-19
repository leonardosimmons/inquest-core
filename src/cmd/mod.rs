#![allow(unused)]
pub mod get;
use get::Get;

use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct CommandOpts {
    tags: Option<Vec<String>>
}

/// `Commands` issued by system
///
/// `Data::Array` should contain all the elements of the command
/// - [element one] - `Origin`
/// - [element two] - Serialized `Command`
pub enum Command {
    Get(Get)
}

impl CommandOpts {
    pub fn new() -> Self {
        CommandOpts { tags: None }
    }

    pub fn tags(&self) -> &Option<Vec<String>> {
        &self.tags
    }
}

impl Default for CommandOpts {
    fn default() -> Self {
        CommandOpts::new()
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Get(get) => write!(f, "Get: {}", serde_json::to_string(get).unwrap()),
        }
    }
}
