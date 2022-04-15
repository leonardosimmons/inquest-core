#![allow(unused)]
use bytes::{Bytes, BytesMut};
use std::path::PathBuf;

use crate::cli::Cli;
use crate::file::File;
use crate::html::{Html, HtmlParser, HtmlDocument};
use crate::parse::{Parse, Parser};
use crate::utils::{Result, Unknown};

const DEFAULT_PROBE_CAPACITY: usize = 4096;

enum Origin {
    Document,
    Url,
}

pub trait FileProbe {}

pub struct Configuration {
    capacity: usize,
}

pub struct DocumentProbe<T>
    where
        T: Parser
{
    capacity: usize,
    paths: Vec<PathBuf>,
    parse: Option<T>,
}

pub struct HttpProbe<T>
where
    T: Parser
{
    capacity: usize,
    urls: Vec<String>,
    parse: Option<T>,
}

pub struct ProbeBuilder<S> {
    state: S,
}

impl<S> ProbeBuilder<S> {
    pub fn new() -> ProbeBuilder<Configuration> {
        ProbeBuilder {
            state: Configuration {
                capacity: DEFAULT_PROBE_CAPACITY,
            },
        }
    }
}

impl ProbeBuilder<Configuration> {
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.state.capacity = capacity;
        self
    }

    pub fn document<T: Parser>(self) -> ProbeBuilder<DocumentProbe<T>> {
        ProbeBuilder {
            state: DocumentProbe {
                capacity: self.state.capacity,
                paths: Vec::new(),
                parse: None,
            },
        }
    }

    pub fn http<T: Parser>(self) -> ProbeBuilder<HttpProbe<T>> {
        ProbeBuilder {
            state: HttpProbe {
                capacity: self.state.capacity,
                urls: Vec::new(),
                parse: None,
            },
        }
    }
}

impl<T> ProbeBuilder<DocumentProbe<T>>
where
    T: Parser
{
    pub async fn html(mut self, path: &str) -> DocumentProbe<Parse<Html>> {
        DocumentProbe {
            parse: Some(
                Parse::<Html>::from(
                    File::new(path, &*Bytes::from(String::with_capacity(self.state.capacity)))
                        .await
                        .text(),
                    String::with_capacity(self.state.capacity).as_bytes()
                )
                    .await
                    .unwrap(),
            ),
            paths: self.state.paths,
            capacity: self.state.capacity,
        }
    }
}

impl<T: Parser> FileProbe for DocumentProbe<T> {}

impl<T> DocumentProbe<T>
where
    T: Parser + HtmlParser,
{

}

impl<T: Parser> FileProbe for HttpProbe<T> {}

/*

pub struct Probe<T> {
    origin: OriginOld,
    parse: Parse<T>,
}

impl Probe<Html> {
    pub async fn init(
        origin: OriginOld,
        paths: Vec<String>,
        mut buff: Vec<Probe<Html>>,
        capacity: Option<usize>,
    ) -> Result<Vec<Probe<Html>>> {
        let mut iter = paths.into_iter();
        loop {
            match iter.next() {
                Some(path) => match origin {
                    OriginOld::Document => match capacity {
                        Some(cap) => buff.push(
                            Probe::from_path(&path, &*Bytes::from(String::with_capacity(cap)))
                                .await
                                .unwrap(),
                        ),
                        None => buff.push(Probe::from_path(&path, &*BytesMut::new()).await.unwrap())
                    },
                    OriginOld::Url => match capacity {
                        Some(cap) => buff.push(Probe::from_url(&path).await.unwrap()),
                        None => buff.push(Probe::from_url(&path).await.unwrap()),
                    },
                },
                None => break,
            };
        }
        Ok(buff)
    }

    async fn from_path(path: &str, mut buf: &[u8]) -> Result<Probe<Html>> {
        Ok(Probe {
            origin: OriginOld::Document,
            parse: Parse::<Html>::from(File::new(path, buf).await.text())?,
        })
    }

    async fn from_url(url: &str) -> Result<Probe<Html>> {
        Ok(Probe {
            origin: OriginOld::Url,
            parse: Parse::<Html>::from_url(url).await?,
        })
    }
}

 */
