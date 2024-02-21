use std::net::{SocketAddr, IpAddr};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    // Define the address to bind to for UDP
    let udp_addr = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8081);

    // Create a UDP socket
    let udp_socket = UdpSocket::bind(&udp_addr).await.unwrap();
    println!("UDP Server listening on: {}", udp_addr);

    // Receive incoming UDP packets in a loop
    loop {
        let mut buf = [0; 1024];
        let (size, client_addr) = udp_socket.recv_from(&mut buf).await.unwrap();

        tokio::spawn(handle_udp_packet(buf[..size].to_vec(), client_addr));
    }
}

async fn handle_udp_packet(data: Vec<u8>, client_addr: SocketAddr) {
    println!("Received UDP packet from {}: {:?}", client_addr, data);

   // response to the client
    let response = "Hello from UDP server";
    tokio::time::sleep(std::time::Duration::from_secs(5)).await; // Simulate some async work
    tokio::spawn(async move {
        udp_socket.send_to(response.as_bytes(), client_addr).await.unwrap();
    });
}
