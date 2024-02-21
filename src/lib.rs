
mod tcp;
mod udp;


pub fn scan(ip: std::net::IpAddr, ports: u16, protocol: &str) {

    match protocol {
        "tcp" => tcp::scan_tcp_port(ip, ports),
        "udp" => udp::scan_udp_port(ip, ports),
        "both" => {
            tcp::scan_tcp_port(ip, ports.clone());
            udp::scan_udp_port(ip, ports);
        },
        _ => println!("Invalid protocol"),
    }
}