#![allow(unused)]

use bytes::{Bytes, BytesMut};
use std::path::PathBuf;

use crate::file::File;
use crate::html::{Html, HtmlParser};
use crate::parse::{Parse, Parser};
use crate::utils::{Result, Unknown};
use crate::cli::Cli;

pub trait Probe {}

// pub struct DocumentProbe<Parser> {
//     paths: Vec<PathBuf>,
//     parse: Parser
// }
//
// pub struct HttpProbe<T> {
//     urls: Vec<String>,
// }
//
// pub struct ProbeMain<Origin>
//     where
//         Origin: Probe,
// {
//     origin: Origin,
//     capacity: Option<usize>,
// }
//
// impl<Origin> ProbeMain<Origin> {
//     pub fn new() -> ProbeMain<Unknown> {
//         ProbeMain {
//             origin: Unknown,
//             capacity: None,
//         }
//     }
// }
//
// impl<T> Probe for DocumentProbe<T> {}
// impl<T> Probe for HttpProbe<T> {}

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