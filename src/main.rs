use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
// use std::net::ToSocketAddrs; commented out for testing purposes
use std::ops::Range;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let target: &str = "scanme.nmap.org"; // Replace with desired target
    let port_range: Range<i32> = 1..1024;
    let total_ports = port_range.len();
    let start = Instant::now();

    println!("🔍 Scanning {} ports on {}...\n", total_ports, target);

    let mut scanned = 0;

    for port in port_range {
        let addr = format!("{}:{}", target, port);
        scanned += 1;

        // Set a timeout for each connection attempt (e.g., 1 second)
        match timeout(Duration::from_secs(1), TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => println!(" Port {} is open", port),
            _ => {} // Closed, unreachable, or timed out
        }

        // Show progress every 100 ports
        if scanned % 100 == 0 || scanned == total_ports {
            let elapsed = start.elapsed().as_secs_f32();
            let avg_time_per_port = elapsed / scanned as f32;
            let estimated_remaining = avg_time_per_port * (total_ports - scanned) as f32;
            println!(
                " Progress: {}/{} | ~{:.1}s remaining",
                scanned,
                total_ports,
                estimated_remaining
            );
        }
    }

    let total_time = start.elapsed().as_secs_f32();
    println!("\n✅ Scan complete in {:.2} seconds", total_time);
}
