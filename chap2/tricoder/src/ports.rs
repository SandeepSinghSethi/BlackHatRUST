use std::{net::{SocketAddr, TcpStream, ToSocketAddrs}, time::Duration};

use rayon::prelude::*;

use crate::{common_ports::MOST_COMMON_PORTS_100, model::{Port, Subdomain}};


// scan_ports will be ran from main.rs file and will run for each subdomain of all subdomains.
pub fn scan_ports(mut subdomain : Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024",subdomain.domain)
        .to_socket_addrs()
        .expect("Port scanner: Creating Socket address")
        .collect();

    if socket_addresses.is_empty(){
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port(socket_addresses[0],*port))
        .filter(|port| port.is_open)
        .collect();

    subdomain

    // this will be called on a thread of the thread pool from the main function of the main.rs file and for each domain , this function is called , and it checks for the port if it is open or not , by returning the subdomain , filling its open_ports entry with the ports which are active , checked by scan_port functions , passed around 100 times , in the second block of this function
}

fn scan_port(mut sock_addr : SocketAddr, port : u16) -> Port{
    let timeout = Duration::from_secs(3);
    sock_addr.set_port(port);

    let is_open = TcpStream::connect_timeout(&sock_addr, timeout).is_ok();

    Port { port, is_open }
}