use std::net::{SocketAddr, TcpStream, UdpSocket, IpAddr};

fn scan_tcp_port(ip: IpAddr, port: u16) {
    let target = SocketAddr::new(ip, port);

    match TcpStream::connect_timeout(&target, std::time::Duration::from_secs(1)) {
        Ok(_) => println!("TCP Port {} is open", port),
        Err(_) => println!("TCP Port {} is closed", port),
    }
}

fn scan_udp_port(ip: IpAddr, port: u16) {
    let target = SocketAddr::new(ip, port);

    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            socket.set_nonblocking(true).unwrap();
            socket.send_to(&[0u8; 0], target).ok();
            let mut buf = [0u8; 1];
            match socket.recv_from(&mut buf) {
                Ok(_) => println!("UDP Port {} is open", port),
                Err(_) => println!("UDP Port {} is closed", port),
            }
        }
        Err(_) => println!("Error creating UDP socket for Port {}", port),
    }
}

fn main() {
    let ip = "127.0.0.1".parse().unwrap(); // Replace with the target IP address
    let ports_to_scan = vec![21,22,23,25,53,80,110,115,135,139,143,194,443,445,1433,3306,3389,5632,5900,8080,25565]; // Add the ports you want to scan

    for &port in &ports_to_scan {
        scan_tcp_port(ip, port);
        scan_udp_port(ip, port);
    }
}
