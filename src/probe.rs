use std::path::PathBuf;

use crate::file::File;
use crate::html::Html;
use crate::parse::Parse;
use crate::utils::Result;

pub enum Origin {
    Document,
    Url
}

pub struct Probe<P> {
    origin: Origin,
    parse: Parse<P>,
}

impl Probe<Html> {
    pub async fn from(paths: Vec<String>, origin: Origin) -> Result<Vec<Probe<Html>>> {
        let mut buf = Vec::new();
        let mut iter = paths.into_iter();
        loop {
            match iter.next() {
                Some(path) => match origin {
                    Origin::Document => buf.push(Probe::from_path(&path).await.unwrap()),
                    Origin::Url => buf.push(Probe::from_url(&path).await.unwrap()),
                },
                None => break,
            }
        }
        Ok(buf)
    }

    async fn from_path(path: &str) -> Result<Probe<Html>> {
        let f = File::from(path).await?;
        Ok(Probe {
            origin: Origin::Document,
            parse: Parse::from(f.text())?,
        })
    }

    async fn from_url(url: &str) -> Result<Probe<Html>> {
        Ok(Probe {
            origin: Origin::Url,
            parse: Parse::from_url(url).await?,
        })
    }

    async fn get_all_links(&self) -> Result<Vec<String>> {
        self.parse.all_links().await
    }

    pub async fn all_links(paths: Vec<PathBuf>) -> Result<Vec<String>> {
        let mut buf = Vec::new();
        let fixed = paths.into_iter().map(|p| Parse::path_to_string(p)).collect();
        let temp = Probe::from(fixed, Origin::Document).await.unwrap();
        let mut probes = temp.into_iter();
        loop {
            match probes.next() {
                Some(probe) => {
                    let temp = probe.get_all_links().await.unwrap();
                    temp.iter().for_each(|link| buf.push(link.clone()))
                }
                None => break,
            }
        }
        Ok(buf)
    }
}
