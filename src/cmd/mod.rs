pub mod get;
use get::Get;

use crate::data::Data;

pub struct CommandOpts {
    tags: Option<Vec<String>>
}

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
