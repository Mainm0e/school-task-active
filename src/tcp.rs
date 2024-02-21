use std::net::{SocketAddr, TcpStream, IpAddr};

pub fn scan_tcp_port(ip: IpAddr, port: u16) {
    let target = SocketAddr::new(ip, port);

    match TcpStream::connect_timeout(&target, std::time::Duration::from_secs(1)) {
        Ok(_) => println!("TCP Port {} is open", port),
        Err(_) => println!("TCP Port {} is closed", port),
    }
}