use std::net::{SocketAddr, UdpSocket, IpAddr};


pub fn scan_udp_port(ip: IpAddr, port: u16) {
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