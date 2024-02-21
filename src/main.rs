use std::env;
use std::net::IpAddr;
use tinyscanner::scan;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
       
    if  args[1] == "--help" {
        print!("
        Usage: cargo run --release -- [OPTIONS] [HOST] [PORT]\n
        Options:\n
        -p\t\t\tRange of ports to scan\n
        -u\t\t\tUDP scan\n
        -t\t\t\tTCP scan\n
        --help\t\t\tPrints help information\n\n");
        std::process::exit(1);
    }

    // Check if there are enough arguments
    if args.len() < 5 {
        eprintln!("Usage: {} [-u | -t] <HOST> -p <PORT>", args[0]);
        std::process::exit(1);
    }

    // Parse command line options
    let mut protocol = "tcp"; // Default protocol is TCP
    let mut ip_str : &str = "";
    let mut ports = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-u" => {
                protocol = "udp";
            }
            "-t" => {
                protocol = "tcp";
            }
            "-p" => {
                i += 1;
                if i < args.len() {
                    match args[i].parse::<u16>() {
                        Ok(port) => {
                            ports.push(port);
                        }
                        Err(_) => {
                            eprintln!("Invalid port number: {}", args[i]);
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Missing port number after -p option");
                    std::process::exit(1);
                }
            }
            _ => {
                ip_str = &args[i];
            }
        }
        i += 1;
    }

        // Check if IP is provided
        if ip_str.is_empty() {
            eprintln!("Missing target IP address");
            std::process::exit(1);
        }
    
        // Parse IP address
        let ip: IpAddr = match ip_str.parse() {
            Ok(ip) => ip,
            Err(_) => {
                eprintln!("Invalid IP address: {}", ip_str);
                std::process::exit(1);
            }
        };
    

    // Check if port is provided
    if ports.is_empty() {
        eprintln!("Missing port number");
        std::process::exit(1);
    }
    // Perform scanning
    for port in ports {
        scan(ip, port, protocol);
    }
}