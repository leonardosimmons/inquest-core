#![allow(unused)]
use bytes::{Bytes, BytesMut};
use std::path::PathBuf;

use crate::cli::Cli;
use crate::file::File;
use crate::html::{Html, HtmlController, HtmlParser};
use crate::parse::{Base, Parse, Parser};
use crate::utils::{Result, Unknown};

const DEFAULT_PROBE_CAPACITY: usize = 4096;

enum Origin {
    Document,
    Url,
}

pub trait Probe {}

pub struct Initialized {
    capacity: usize,
}

pub struct DocumentProbe<Parser> {
    capacity: usize,
    paths: Vec<PathBuf>,
    parse: Option<Parser>,
}

pub struct HttpProbe<Parser> {
    capacity: usize,
    urls: Vec<String>,
    parse: Option<Parser>,
}

pub struct ProbeMain<S> {
    state: S,
}

impl<S> ProbeMain<S> {
    pub fn new() -> ProbeMain<Initialized> {
        ProbeMain {
            state: Initialized {
                capacity: DEFAULT_PROBE_CAPACITY,
            },
        }
    }
}

impl ProbeMain<Initialized> {
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.state.capacity = capacity;
        self
    }

    pub fn document<T>(self) -> ProbeMain<DocumentProbe<T>> {
        ProbeMain {
            state: DocumentProbe {
                capacity: self.state.capacity,
                paths: Vec::new(),
                parse: None,
            },
        }
    }

    pub fn http<T>(self) -> ProbeMain<HttpProbe<T>> {
        ProbeMain {
            state: HttpProbe {
                capacity: self.state.capacity,
                urls: Vec::new(),
                parse: None,
            },
        }
    }
}

impl<Parser> Probe for DocumentProbe<Parser> {}

impl<Parser> DocumentProbe<Parser>
where
    Parser: HtmlParser + HtmlController,
{
    pub async fn html(mut self, path: &str) -> DocumentProbe<Parse<Html>> {
        DocumentProbe {
            parse: Some(
                Parse::from(
                    File::new(path, &*Bytes::from(String::with_capacity(self.capacity)))
                        .await
                        .text(),
                )
                .unwrap(),
            ),
            paths: self.paths,
            capacity: self.capacity,
        }
    }
}

impl<T> Probe for HttpProbe<T> {}

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
