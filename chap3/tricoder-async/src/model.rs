use serde::Deserialize;

#[derive(Debug,Clone)]
pub struct Subdomain{
    pub domain : String,
    pub open_ports : Vec<Port>,
}

#[derive(Debug,Clone)]
pub struct Port{
    pub port: u16,
    pub is_open: bool,
}

#[derive(Debug,Deserialize,Clone)] // this entry is returned by https://crt.sh/?q=%.google.com&output=json
pub struct CrtshEntry{
    pub name_value : String,
}