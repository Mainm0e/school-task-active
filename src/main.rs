
use tinyscanner::scan;
#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".parse().unwrap(); // Replace with the target IP address
    /* let ports = vec![21,22,23,25,53,80,110,115,135,139,143,194,443,445,1433,3306,3389,5632,5900,8080,25565]; // Add the ports you want to scan */
    let ports = vec![8080, 8081]; // Add the ports you want to scan    
    scan(ip, ports).await;
}
