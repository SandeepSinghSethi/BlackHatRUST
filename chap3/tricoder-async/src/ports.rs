use std::{net::{SocketAddr,  ToSocketAddrs}, time::Duration};

// use rayon::prelude::*;

use futures::StreamExt;
use tokio::{net::TcpStream, sync::mpsc};

use crate::{common_ports::MOST_COMMON_PORTS_100, model::{Port, Subdomain}};


// scan_ports will be ran from main.rs file and will run for each subdomain of all subdomains.
pub async fn scan_ports(concurrency: usize, subdomain : Subdomain) -> Subdomain {
    let mut ret = subdomain.clone();
    // make 1024 -> 65535 for scanning all ports , when using all ports array
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024",subdomain.domain)
        .to_socket_addrs()
        .expect("Port scanner: Creating Socket address")
        .collect();

    if socket_addresses.is_empty(){
        return subdomain;
    }

    let socket_address = socket_addresses[0];

    // subdomain.open_ports = MOST_COMMON_PORTS_100
    //     .into_par_iter()
    //     .map(|port| scan_port(socket_addresses[0],*port))
    //     .filter(|port| port.is_open)
    //     .collect();

    let (input_tx,input_rx) = mpsc::channel::<u16>(concurrency);
    let (output_tx,output_rx) = mpsc::channel::<Port>(concurrency);
    // this will be called on a thread of the thread pool from the main function of the main.rs file and for each domain , this function is called , and it checks for the port if it is open or not , by returning the subdomain , filling its open_ports entry with the ports which are active , checked by scan_port functions , passed around 100 times , in the second block of this function

    // all ports are send to the channel 
    tokio::spawn(async move{
        for port in MOST_COMMON_PORTS_100{
            let _ = input_tx.send(*port).await;
        }
    });

    // full process
        // 1. all ports are sent to channels via input_tx
        // 2. all ports are received asynchronously and process via scan_port 
        // 3. all processed ports are sent to channel via output_tx 
        // 4. all process ports are collected via output_rx...
    // all inputs are read and processed asynchronously
    let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
    input_rx_stream
        .for_each_concurrent(concurrency, |port|{
            let output_tx = output_tx.clone();
            async move{
                let port = scan_port(socket_address, port).await;
                if port.is_open{
                    let _ = output_tx.send(port).await;
                }
            }
        })
        .await;

    // now close the output_tx channel , as no other transmission of output is required to be sent to it.
    drop(output_tx);

    // now get all active ports from output_rx
    let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);
    ret.open_ports = output_rx_stream.collect().await;

    ret
}

async fn scan_port(mut sock_addr : SocketAddr, port : u16) -> Port{
    let timeout = Duration::from_secs(3);
    sock_addr.set_port(port);

    // let is_open = TcpStream::connect_timeout(&sock_addr, timeout).is_ok();
    let is_open = matches!(tokio::time::timeout(timeout,TcpStream::connect(&sock_addr)).await,
    Ok(Ok(_)),
);
    Port { port, is_open }
}