
use std::{net::{SocketAddr, ToSocketAddrs}, time::Duration};
use futures::{StreamExt, stream};
use tokio::net::TcpStream;

use crate::{common_ports::MOST_COMMON_PORTS, modules::{Port, Subdomain}};



pub async fn scan_ports(concurrency: usize, mut subdomain : Subdomain) -> Subdomain{
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024",subdomain.domain)
        .to_socket_addrs()
        .expect("Port scanner: setting up socket addresses")
        .collect();
    
    if socket_addresses.len() == 0{
        return subdomain;
    }

    let socket_addr = socket_addresses[0];
    
    //? using move just because we have to pass a copy of a value to the execution flow, else simple async will make the async task run.
    subdomain.open_ports = stream::iter(MOST_COMMON_PORTS.into_iter())
        .map(|port| async move{
            let port = scan_port(socket_addr, *port).await;
            if port.is_open{
                return Some(port);
            }
            None
        })
        .buffer_unordered(concurrency)
        .filter_map(|port| async move {port})
        .collect()
        .await;

      subdomain
}

pub async fn scan_port(mut socket_addr: SocketAddr,port : u16) -> Port{
    let timeout = Duration::from_secs(3);
    socket_addr.set_port(port);
    // let socket_addr =  SocketAddr::new(ip, port);

    let isopen = matches!(
        tokio::time::timeout(timeout, TcpStream::connect(&socket_addr)).await,
        Ok(Ok(_))
    );

    Port{
        port: port,
        is_open:isopen,
        findings: Vec::new(),
    }
    
}
