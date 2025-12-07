use crate::Error;
use async_trait::async_trait;
use reqwest::Client;

mod http;
mod subdomains;

pub trait Module{
    fn name(&self) ->String;
    fn description(&self) ->String;
}

/// SUBDOMAINS///
/// 
// with this subdomainmodule will have 3 func: name,desc, enumerate...
#[async_trait]
pub trait SubdomainModule: Module{ // subdomainModule requires module to be init .
    fn enumerate(&self,domain: &str) ->Result<Vec<String>,Error>;
}

#[derive(Debug,Clone)]
pub struct Subdomain{
    pub domain: String,
    pub open_ports: Vec<Port>,
}

#[derive(Debug,Clone)]
pub struct Port{
    pub port :u16,
    pub is_open: bool,
    pub findings: Vec<HttpFinding>,
}

/// //// //// ///
/// //// //// ///

/// HTTP ///
/// 
// with this HttpModule trait will have 3 functions: name, desc, scan
#[async_trait]
pub trait HttpModule: Module{
    fn scan(&self,http_client: &Client,endpoint: &str)-> Result<Option<HttpFinding>,Error>;
}


#[derive(Debug,Clone)]
pub enum HttpFinding{
    DsStoreFileDisclosure(String),
    DotEnvFileDisclosure(String),
    DirectoryListingDisclosure(String),
    TraefikDashboardUnauthenticatedAccess(String),
    PrometheusDashboardUnauthenticatedAccess(String),
    KibanaUnauthenticatedAccess(String),
    GitlabOpenRegistrations(String),
    GitHeadDisclosure(String),
    GitDirectoryDisclosure(String),
    GitConfigDisclosure(String),
    EtcdUnauthenticatedAccess(String),
    Cve2017_9506(String),
    Cve2018_7600(String),
    ElasticsearchUnauthenticatedAccess(String),
}

/// //// //// ///
/// //// //// ///
