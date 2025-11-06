use std::{
    env, time::{Duration, Instant}
};
use futures::{stream, StreamExt};
// use rayon::prelude::*;
// use rayon::iter::IntoParallelIterator;
use reqwest::{redirect, Client};

// must include these modules in main.rs to make rust-analyzer work in these files.
mod error;
mod common_ports;
mod ports;
mod subdomains;
mod model;

//using pub use just because for global scope stuff....
use model::Subdomain;
pub use error::Error;


#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<(),anyhow::Error>{

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();
    // println!("{target}");

    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    // we create now a thread pool for parallel execution of the task on various threads
    // let pool = rayon::ThreadPoolBuilder::new()
    //     .num_threads(128)
    //     .build()
    //     .expect("Failed to allocated threads to rayon!!");

    // pool.install(|| {
    //     // for some reason the below code is crashing if not using serde_json , and after using it , it works completely fine ?
    //     match subdomains::enumerate(&http_client, target) {
    //        Ok(subs) => {
    //         let scan_result : Vec<Subdomain> = subs
    //             .into_par_iter()
    //             .map(ports::scan_ports)
    //             .collect();

    //         for subdomain in scan_result{
    //             let active_port = subdomain.open_ports;
    //             let name = subdomain.domain;

    //             println!("Domain: {name}");
    //             println!("  --Open Ports --");
    //             print!("        [");
    //             for port in active_port{
    //                 print!(" {} ,",port.port);
    //             }
    //             println!(" ]");
    //         }
            
    //        } 
    //        Err(e) => {
    //         eprintln!("Error during subdomain enumeration!! : {}",e);
    //        }
    //     }
            
        
    // });

    let ports_concurrency = 200;
    let subdomain_concurrency = 200;
    let scan_start = Instant::now();

    let subdomains = subdomains::enumerate(&http_client, target).await?;

    let scan_result: Vec<Subdomain> = stream::iter(subdomains.into_iter())
        .map(|subdomain| ports::scan_ports(ports_concurrency, subdomain))
        .buffer_unordered(subdomain_concurrency)
        .collect().await;


    let elapsed_time = scan_start.elapsed();
    println!("Scan Completed in {:?}",elapsed_time);

    for subdomain in scan_result{
        println!("{}",&subdomain.domain);
        print!("OPEN PORTS : [");
        for port in &subdomain.open_ports{
            print!("{}  ,",port.port);
        }
        println!("] ");
    }
   
    Ok(())
}
