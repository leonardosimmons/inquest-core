use select::predicate::Predicate;
use std::default::Default as StdDefault;

use crate::html::{Headers, Html, HtmlDocument, HtmlParser, HtmlTag};
use crate::parse::{Default, FromPath, FromUrl, Parse, Parser};
use crate::utils::Result;

const DEFAULT_BUFFER_CAPACITY: usize = 4 * 1024 * 1024; // 4mb

pub struct DocumentProbe<T>
where
    T: Parser,
{
    capacity: usize,
    path: String,
    parse: T,
}

pub struct HttpProbe<T>
where
    T: Parser,
{
    url: String,
    parse: T,
}

pub struct Probe;

impl Probe {
    pub fn new() -> Probe { Probe {} }
}

impl Probe {
    pub fn document(self) -> DocumentProbe<Parse<Default>> {
        DocumentProbe {
            capacity: DEFAULT_BUFFER_CAPACITY,
            path: String::new(),
            parse: Parse::<Default>::default(),
        }
    }

    pub fn http(self) -> HttpProbe<Parse<Default>> {
        HttpProbe {
            url: String::new(),
            parse: Parse::<Default>::default(),
        }
    }
}

impl<T> DocumentProbe<T>
where
    T: Parser,
{
    pub fn buffer(self, capacity: usize) -> Self {
        Self {
            capacity,
            parse: self.parse,
            path: self.path,
        }
    }
}

impl<T> DocumentProbe<T>
where
    T: Parser + FromPath + Send,
{
    pub async fn from(mut self, path: &str) -> Result<Self> {
        Ok(Self {
            capacity: self.capacity,
            path: path.to_string(),
            parse: self.parse.from(path, self.capacity).await?,
        })
    }
}

impl DocumentProbe<Parse<Default>> {
    pub fn html(self) -> DocumentProbe<Parse<Html>> {
        DocumentProbe {
            capacity: self.capacity,
            parse: Parse::new(Html::default()),
            path: self.path,
        }
    }
}

impl<T> DocumentProbe<Parse<T>>
where
    T: HtmlDocument,
{
    pub fn all_links(&self) -> Result<Vec<String>> {
        self.parse.all_links()
    }

    pub fn all_headers(&self) -> Result<Vec<Headers>> {
        self.parse.all_headers(vec![])
    }
}

impl<T> HtmlParser for DocumentProbe<T>
where
    T: Parser + HtmlParser,
{
    fn descriptions(&self) -> Result<Vec<String>> {
        self.parse.descriptions()
    }

    fn header(&self, header: HtmlTag) -> Result<Headers> {
        self.parse.header(header)
    }

    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        self.parse.links(predicate)
    }

    fn page_title(&self) -> Result<Vec<String>> {
        self.parse.page_title()
    }
}

impl<T> HttpProbe<T>
where
    T: Parser + FromUrl + Send,
{
    pub async fn from(mut self, url: &str) -> Result<Self> {
        Ok(Self {
            url: url.to_string(),
            parse: self.parse.from(url).await?,
        })
    }
}

impl HttpProbe<Parse<Default>> {
    pub fn html(self) -> HttpProbe<Parse<Html>> {
        HttpProbe {
            parse: Parse::new(Html::default()),
            url: self.url,
        }
    }
}

impl<T> HtmlParser for HttpProbe<T>
where
    T: Parser + HtmlParser,
{
    fn descriptions(&self) -> Result<Vec<String>> {
        self.parse.descriptions()
    }

    fn header(&self, header: HtmlTag) -> Result<Headers> {
        self.parse.header(header)
    }

    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>> {
        self.parse.links(predicate)
    }

    fn page_title(&self) -> Result<Vec<String>> {
        self.parse.page_title()
    }
}

impl<T> HttpProbe<Parse<T>>
where
    T: HtmlDocument,
{
    pub fn all_links(&self) -> Result<Vec<String>> {
        self.parse.all_links()
    }

    pub fn all_headers(&self) -> Result<Vec<Headers>> {
        self.parse.all_headers(Vec::new())
    }
}
