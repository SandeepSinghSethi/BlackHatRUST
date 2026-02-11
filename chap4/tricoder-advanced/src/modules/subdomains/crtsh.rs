use std::collections::HashSet;

use crate::Error;
use crate::modules::{Module,SubdomainModule};
use serde::{Serialize,Deserialize};
use async_trait::async_trait;

pub struct Crtsh{}

impl Crtsh{
    pub fn new() -> Self {
        Crtsh{}
    }
}

impl Module for Crtsh{
    fn name(&self) -> String{
        String::from("subdomains/crtsh")
    }

    fn description(&self) -> String{
        String::from("Uses crt.sh to find subdomains ...")
    }
}

#[derive(Clone,Debug,Serialize,Deserialize)]
struct CrtShEntry{
    name_value: String,
}


#[async_trait]
impl SubdomainModule for Crtsh{
    async fn enumerate(&self,domain: &str) -> Result<Vec<String>,Error>{
        let url = format!("https://crt.sh/?q=%25.{}&output=json",domain);
        let res = reqwest::get(&url).await?;

        if !res.status().is_success(){
            return Err(Error::InvalidHTTPResponse(self.name()));
        }

        let crtsh_entries: Vec<CrtShEntry> = match res.json().await {
            Ok(info) => info,
            Err(_) => return Err(Error::InvalidHTTPResponse(self.name())),
        };

        let subdomains: HashSet<String> = crtsh_entries
            .into_iter()
            .map(|entry| {
                entry
                    .name_value
                    .split('\n')
                    .map(|subdomain| subdomain.trim().to_string())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .filter(|subdomain| !subdomain.contains('*'))
            .collect();

        Ok(subdomains.into_iter().collect())

    }
}