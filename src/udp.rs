use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

pub fn scan_udp_port(ip: IpAddr, port: u16) {
    let target = SocketAddr::new(ip, port);

    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            socket.set_nonblocking(true).unwrap(); // Set the socket to non-blocking

            let payload: &[u8] = &[0u8; 0];

            // Send an empty UDP packet to the target
            if let Ok(_) = socket.send_to(payload, target) {
                let start_time = Instant::now();
                let timeout_duration = Duration::from_secs(1);

                loop {
                    // Attempt to receive a response with a non-blocking operation
                    let mut buf = [0u8; 1024]; // Adjust the buffer size as needed
                    match socket.recv_from(&mut buf) {
                        Ok((size, _)) => {
                            // Process the received data if needed
                            println!("Port {} is open | Received {} bytes", port, size);
                            break;
                        }
                        Err(_) => {
                            if Instant::now() - start_time >= timeout_duration {
                                // Timeout reached, no response
                                println!("Port {} is open/filtered (no response)", port);
                                break;
                            }
                            // Optionally sleep for a short duration to avoid busy waiting
                            std::thread::sleep(Duration::from_millis(10));
                        }
                    }
                }
            } else {
                println!("Error sending UDP packet for Port {}", port);
            }
        }
        Err(_) => println!("Error creating UDP socket for Port {}", port),
    }
}
