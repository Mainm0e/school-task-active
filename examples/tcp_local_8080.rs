use std::net::{SocketAddr, IpAddr};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Define the address to bind to
    let addr = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8080);

    // Create a TCP listener
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on: {}", addr);

    // Accept incoming TCP connections in a loop
    while let Ok((socket, client_addr)) = listener.accept().await {
        // Spawn a new asynchronous task to handle each incoming connection
        tokio::spawn(handle_connection(socket, client_addr));
    }
}

async fn handle_connection(socket: TcpStream, client_addr: SocketAddr) {
    println!("Accepted connection from: {}", client_addr);

    // Handle the connection logic here
    // For example, you can read from and write to the socket using `tokio::io::AsyncRead` and `tokio::io::AsyncWrite`.

    // In this example, we just print a simple message and close the connection.
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await; // Simulate some async work
        println!("Handling connection from: {}", client_addr);
    });
}
