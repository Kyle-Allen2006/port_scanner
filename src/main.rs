use tokio::net::TcpStream;
use std::time::Instant;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    let target = "scanme.nmap.org";
    let ports: Vec<u16> = (1..1024).collect();
    let total_ports = ports.len();
    let start = Instant::now();

    println!("Concurrently scanning {} ports on {}...\n", total_ports, target);

    let tasks: Vec<_> = ports
        .iter()
        .map(|&port| {
            let addr = format!("{}:{}", target, port);
            tokio::spawn(async move {
                if TcpStream::connect(&addr).await.is_ok() {
                    Some(port)
                } else {
                    None
                }
            })
        })
        .collect();

    let results = join_all(tasks).await;

    let mut open_ports = Vec::new();
    for result in results {
        if let Ok(Some(port)) = result {
            open_ports.push(port);
        }
    }

    let elapsed = start.elapsed().as_secs_f32();
    println!("\nScan complete in {:.2} seconds", elapsed);

    if open_ports.is_empty() {
        println!("No open ports found.");
    } else {
        println!("Open ports:");
        for port in open_ports {
            println!("  - Port {}", port);
        }
    }
}
