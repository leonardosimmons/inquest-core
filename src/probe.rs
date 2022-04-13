#![allow(unused)]
use crate::file::File;
use crate::html::{Html, HtmlParser};
use crate::parse::Parse;
use crate::utils::Result;
use bytes::{Bytes, BytesMut};

pub enum Origin {
    Document,
    Url,
}

pub struct Probe<T> {
    origin: Origin,
    parse: Parse<T>,
}

impl Probe<Html> {
    pub async fn from(
        origin: Origin,
        paths: Vec<String>,
        mut probes: Vec<Probe<Html>>,
        capacity: Option<usize>,
    ) -> Result<Vec<Probe<Html>>> {
        let mut iter = paths.into_iter();
        loop {
            match iter.next() {
                Some(path) => match origin {
                    Origin::Document => match capacity {
                        Some(cap) => probes.push(
                            Probe::from_path(&path, &*Bytes::from(String::with_capacity(cap)))
                                .await
                                .unwrap(),
                        ),
                        None => {
                            probes.push(Probe::from_path(&path, &*BytesMut::new()).await.unwrap())
                        }
                    },
                    Origin::Url => {}
                },
                None => break,
            };
        }
        Ok(probes)
    }

    async fn from_path(path: &str, mut buf: &[u8]) -> Result<Probe<Html>> {
        Ok(Probe {
            origin: Origin::Document,
            parse: Parse::from(File::new(path, buf).await.text())?,
        })
    }

    async fn from_url(url: &str) -> Result<Probe<Html>> {
        Ok(Probe {
            origin: Origin::Url,
            parse: Parse::<Html>::from_url(url).await?,
        })
    }
}
