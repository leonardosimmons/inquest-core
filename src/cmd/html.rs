use std::str::FromStr;
use select::predicate::{Name, Predicate};
use crate::error::{Error, ErrorKind};

use crate::html::{Headers, HtmlTag};

pub enum HtmlCommand {
    AllHeaders,
    AllLinks,
    Bytes,
    Document,
    Description,
    FixLink(String),
    FromFile(String),
    FromText(String),
    FromUrl(String),
    Headers(Headers),
    HeaderTag(HtmlTag),
    Links(Box<dyn Predicate + Send>),
    PageTitle,
    Text,
    Unknown
}

impl FromStr for HtmlCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HtmlCommand::*;
        use crate::html::Headers::Invalid;

        match s {
            "all-headers" => Ok(AllHeaders),
            "all-links" => Ok(AllLinks),
            "bytes" => Ok(Bytes),
            "document" => Ok(Document),
            "description" => Ok(Description),
            "fix-link" => Ok(FixLink("".to_string())),
            "from-file" => Ok(FromFile("".to_string())),
            "from-text" => Ok(FromText("".to_string())),
            "from-url" => Ok(FromUrl("".to_string())),
            "headers" => Ok(Headers(Invalid)),
            "header-tag" => Ok(HeaderTag(HtmlTag::Invalid)),
            "links" => Ok(Links(Box::new(Name("")))),
            "page-title" => Ok(PageTitle),
            "text" => Ok(Text),
            _ => Err(Error::from(ErrorKind::Unknown))
        }
    }
}

impl FromStr for Box<HtmlCommand> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HtmlCommand::*;
        use crate::html::Headers::Invalid;

        match s {
            "all-headers" => Ok(Box::new(AllHeaders)),
            "all-links" => Ok(Box::new(AllLinks)),
            "bytes" => Ok(Box::new(Bytes)),
            "document" => Ok(Box::new(Document)),
            "description" => Ok(Box::new(Description)),
            "fix-link" => Ok(Box::new(FixLink("".to_string()))),
            "from-file" => Ok(Box::new(FromFile("".to_string()))),
            "from-text" => Ok(Box::new(FromText("".to_string()))),
            "from-url" => Ok(Box::new(FromUrl("".to_string()))),
            "headers" => Ok(Box::new(Headers(Invalid))),
            "header-tag" => Ok(Box::new(HeaderTag(HtmlTag::Invalid))),
            "links" => Ok(Box::new(Links(Box::new(Name(""))))),
            "page-title" => Ok(Box::new(PageTitle)),
            "text" => Ok(Box::new(Text)),
            _ => Err(Error::from(ErrorKind::Unknown))
        }
    }
}
