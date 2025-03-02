mod error;
mod model;

use crate::model::Port;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::sync::{mpsc, Arc};
use threadpool::ThreadPool;


pub fn scan_port(socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let mut address = socket_address;
    address.set_port(port);

    let is_open = TcpStream::connect_timeout(&address, timeout).is_ok();

    Port { port, is_open }
}

pub fn scan_ports(subdomain: String) -> Vec<Port> {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.is_empty() {
        return Vec::new();
    }

    let socket_address = socket_addresses[0];
    let pool = ThreadPool::new(10);
    let (tx, rx) = mpsc::channel();


    let ports = Arc::new(MOST_COMMON_PORTS_100.to_vec());

    for &port in ports.iter() {
        let tx = tx.clone();
        let socket_address = socket_address.clone();
        // let ports = Arc::clone(&ports);

        pool.execute(move || {

            let result = scan_port(socket_address, port);
            tx.send(result).expect("Failed to send result");
        });
    }

    drop(tx);


    let open_ports: Vec<Port> = rx.iter().filter(|port| port.is_open).collect();
    open_ports
}

const MOST_COMMON_PORTS_100: [u16; 100] = [
    80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111, 995, 993,
    5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179, 1026, 2000,
    8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646, 5000, 5631,
    631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357, 427, 49156,
    543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432, 1900, 3986,
    13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37,
];

fn main() {
    let subdomain = "agetic.gob.bo".to_string();
    let open_ports = scan_ports(subdomain);

    println!("Open ports: {:?}", open_ports);
}
