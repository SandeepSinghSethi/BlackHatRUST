use std::collections::HashSet;
use std::time::Instant;

use clap::{Parser,ValueEnum,Subcommand,Args};
use futures::{StreamExt, stream};

use crate::modules::{Subdomain, SubdomainModule};//,HttpModule};
use crate::{Error, dns, modules, ports};

#[derive(Parser)]
#[command(
    name= "Tricoder [->][^][<-]",
    version="v1.0.0",
    author="test developer",
    about="A simple async port scanner tool to simplify the process of initial reconnaisance.",
    long_about= None,
)]
pub struct Cli{
    #[arg(short,long,action=clap::ArgAction::Count)]
    verbose: u8,

    #[arg(long,default_value="info",env="LOG_LEVEL")]
    log_level : LogLevel,

    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands{
    #[command(name="modules",about="List all modules")]
    Modules(ModuleStruct),

    #[command(name="scan",about="Scans the given target")]
    Scan(ScanStruct),
}

#[derive(Args)]
pub struct ModuleStruct{
    //left for - if any args are required...
}

#[derive(Args)]
pub struct ScanStruct{
    #[arg(short,long,required=true)]
    pub target: String,
}

#[derive(ValueEnum,Clone)]
pub enum LogLevel {
    INFO,DEBUG,ERROR,
}

// functions....
pub fn modules()  {
    log::debug!("In the modules func.");
    let subdomain_modules = modules::all_subdomains_module();

    println!("All subdomain Modules ...");
    for m in subdomain_modules{
        println!(" {} \t\t: {}",m.name(),m.description())
    }
}

pub async fn scan(target:&str) -> Result<(), Error>{
    log::debug!("In the scan func.");
    
    // println!("Target: {}",target.target);
    log::info!("Scanning: {}",target);

    // let runtime = tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap();
    // my stupid ass was creating an async runtime with an async runtime (bcz tokio::main in main), so thread was panicking....
    let dns_resolver = dns::new_resolver().await;

    let subdomains_concurrency = 20;
    let dns_concurrency = 100;
    let ports_concurrency = 200;
    let scan_start = Instant::now();

    let subdomain_module = modules::all_subdomains_module();
    
    // runtime.block_on(async move{
        let mut subdomains : Vec<String>= stream::iter(subdomain_module.into_iter())
            .map(|module| async move {
                match module.enumerate(&target).await {
                    Ok(new_subdomains) => Some(new_subdomains),
                    Err(err) => {
                        log::error!("subdomains:/{}: {}",module.name(),err);
                        None
                    }
                }
            })
            .buffer_unordered(subdomains_concurrency)
            .filter_map(|subd| async move{
                subd
            })
            .collect::<Vec<Vec<String>>>()
            .await
            .into_iter()
            .flatten()
            .collect();

        subdomains.push(target.to_string());


        let subdomains : Vec<Subdomain>= HashSet::<String>::from_iter(subdomains.into_iter())
            .into_iter()
            .filter(|subdomain| subdomain.contains(target))
            .map(|domain| Subdomain{
                domain,
                open_ports: Vec::new(),
            })
            .collect();

        log::info!("Found subdomains: {}",subdomains.len());
        log::info!("Printing all of them: ");
        for subdomain in subdomains.clone(){
            println!("- {}",subdomain.domain);
        }

        // resolving unresolvable subdomains
        let subdomains : Vec<Subdomain>  = stream::iter(subdomains.into_iter())
            .map(|domain| dns::resolves(&dns_resolver, domain))
            .buffer_unordered(dns_concurrency)
            .filter_map(|domain| async move {domain})
            .collect()
            .await;
        
        // scanning ports ..
        let subdomains: Vec<Subdomain> = stream::iter(subdomains.into_iter())
            .map(|subd| {
                ports::scan_ports(ports_concurrency, subd)
            })
            .buffer_unordered(1)
            .collect()
            .await;


        for subdomain in subdomains{
            println!("{}",subdomain.domain);
            print!("\t[ ");
            for port in subdomain.open_ports{
                print!("{} ,",port.port)
            }
            print!("\t] ");
        }
    // });

    let scan_end = scan_start.elapsed();
    log::info!("Total time taken: {:?}",scan_end);

    Ok(())
}

