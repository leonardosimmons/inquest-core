use bytes::Bytes;
use serde::{Serialize, Deserialize};
use crate::cmd::CommandOpts;
use crate::data::{Data, DataController, Origin};

#[derive(Serialize, Deserialize)]
pub struct Get {
    origin: Origin,
    route: &'static str,
    opts: Option<CommandOpts>,
}

impl Get {
    pub fn new() -> Self {
        Self {
            origin: Origin::Path,
            route: "",
            opts: None,
        }
    }

    pub fn config(self, options: CommandOpts) -> Self {
        Self {
            origin: self.origin,
            route: self.route,
            opts: Some(options)
        }
    }

    pub fn set(self, origin: Origin, route: &'static str) -> Self {
        Self {
            opts: self.opts,
            origin, route,
        }
    }

    pub fn options(&self) -> &Option<CommandOpts> {
        &self.opts
    }

    pub fn origin(&self) -> &Origin {
        &self.origin
    }

    pub fn route(&self) -> &str {
        self.route
    }

    pub fn into_data(self) -> Data {
        let mut data = Data::array();
        data.push_bulk(Bytes::from("get".as_bytes()));
        data.push_bulk(Bytes::from(serde_json::to_string(&self).unwrap()));
        data
    }
}

impl Default for Get {
    fn default() -> Self {
        Get::new()
    }
}
