use std::net::{SocketAddr, IpAddr};
use tokio::net::UdpSocket;


struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    async fn new(addr: &str) -> Result<UdpServer, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(UdpServer {
            socket,
        })
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = [0; 1024];
        loop {
           /*  let (size, client_addr) = self.socket.recv_from(&mut buf).await?;
            self.handle_udp_packet(buf[..size].to_vec(), client_addr).await; */
            match self.socket.recv_from(&mut buf).await {
                Ok((size, client_addr)) => {
                    self.handle_udp_packet(buf[..size].to_vec(), client_addr).await;
                },
                Err(e) => {
                    println!("Error receiving from socket; error = {:?}", e);
                }
                
            }
        }
    }

    async fn handle_udp_packet(&self, data: Vec<u8>, client_addr: SocketAddr) {
        println!("Received UDP packet from {}: {:?}", client_addr, data);
        let response_msg = "Hello from the server!";
        let _ = self.socket.send_to(response_msg.as_bytes(), client_addr).await;
        println!("Sent UDP response to {}", client_addr);

    }

}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8081";
    let mut server = UdpServer::new(addr).await.unwrap();
    server.run().await.unwrap();
}
