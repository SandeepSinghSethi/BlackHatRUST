use std::{collections::HashSet};

use serde::{Serialize,Deserialize};
use url::Url;
use crate::modules::{Module, SubdomainModule};
use crate::Error;
use async_trait::async_trait;

pub struct WebArchive {}

impl WebArchive {
    pub fn new() -> Self{
        WebArchive{}
    }
}

impl Module for WebArchive{
    fn name(&self) ->String {
        String::from("subdomains/web_archive")
    }

    fn description(&self) ->String {
        String::from("use webarchive.org to find domains...")
    }
}

#[derive(Clone,Debug,Deserialize,Serialize)]
struct WebArchiveContent(Vec<Vec<String>>);

#[async_trait]
impl SubdomainModule for WebArchive{
    async fn enumerate(&self,domain : &str) -> Result<Vec<String>,Error> {
        let url : String = format!("https://web.archive.org/cdx/search/cdx?matchType=domain&fl=original&output=json&collapse=urlkey&url={}",domain);
        let res = reqwest::get(url).await?;

        if !res.status().is_success(){
            return Err(Error::InvalidHTTPResponse(self.name()))
        }

        let web_archive_urls : WebArchiveContent = match res.json().await{
            Ok(info) => info,
            Err(_) => return Err(Error::InvalidHTTPResponse(self.name()))
        };

        let subdomains: HashSet<String> = web_archive_urls
            .0
            .into_iter()
            .flatten()
            .filter_map(|domain| {
                Url::parse(&domain)
                    .map_err(|err| {
                        log::error!("{} : Error parsing url : {}", self.name(),err);
                        err
                    })  
                    .ok()
            })
            .filter_map(|url| url.host_str().map(|host| host.to_string()))
            .collect();
            
        
        Ok(subdomains.into_iter().collect())
    }
}