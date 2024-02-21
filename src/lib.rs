
mod tcp;
mod udp;


pub async fn scan(ip: std::net::IpAddr, ports: Vec<u16>) {
    for &port in &ports {
        tcp::scan_tcp_port(ip, port);
        udp::scan_udp_port(ip, port);
    }
}