use crate::Error;
use crate::modules::{Module,SubdomainModule};
use serde::{Serialize,Deserialize};
use async_trait::async_trait;

pub struct Crtsh{}

impl Crtsh{
    fn new(&self) -> Self {
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
    fn enumerate(&self,domain: &str) -> Result<Vec<String>,Error>{

    }
}