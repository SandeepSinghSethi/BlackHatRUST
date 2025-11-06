use crate::{
    model::{CrtshEntry, Subdomain},
    Error
};
use reqwest::blocking::Client;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, Resolver};
use std::{collections::HashSet, time::Duration};


pub fn enumerate(http_client :&Client, target: &str) -> Result<Vec<Subdomain> ,Error> {
    let entries: Vec<CrtshEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;
        
    println!("CODE FROM enumerate");    

    let mut subdomains: HashSet<String> = entries
        .into_iter() // getting values one by one using it as a iterator
        .flat_map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain| subdomain != target)
        .filter(|subdomain| !subdomain.contains("*"))
        .collect();
    // what we have done above is queried the crt.sh api and get the data in json , and using cargo.toml features in reqwest , we have implemented json , so that we can work with json , and rustls-tls , to work with https endpoints, 
    
    // then after getting json data , which looks like this of the field {name_value}:"adwords.google.com\nadwords.google.com.ar\nadwords.google.com.au\nadwords.google.com.br\nadwords.google.com.cn\nadwords.google.com.gr\nadwords.google.com.hk\nadwords.google.com.ly\nadwords.google.com.mx\nadwords.google.com.my\nadwords.google.com.pe\nadwords.google.com.ph\nadwords.google.com.pk\nadwords.google.com.ru\......
    
    // we converted this data into iterator , which return value of name_value for each entry in the json data . and then converted it into a flat map which normalizes the name_value data string with some modifications like : split('\n'), collecting all strings in vec<str>

    // then after doing all those stuff final filtering is done 
        // if it is equal to target (as a lot of entry will contain same string as target)
        // then removed any wildcards , if so , to get valid subdomains

    subdomains.insert(target.to_string()); // adding the target at last , as it is also a valid subdomain i.e. WWW.target.com , here WWW is subdomain

    // now we have hashset of string , which we can query with its key ?? not feasible , so converting it into a vector of subdomains !!check out model.rs

    let subdomains : Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain{
            domain,
            open_ports : Vec::new(),
        })
        .filter(resolves) // it collects reachable subdomains
        .collect();

    Ok(subdomains)
}

pub fn resolves(domain: &Subdomain) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
    .expect("Subdomain resolver : building dns client");

    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}