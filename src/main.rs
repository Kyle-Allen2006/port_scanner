use tokio::net::TcpStream;


#[tokio::main]
async fn main() {
    let target = "scanme.nmap.org"; //This can be changed to any IP or domain
    let port_range = 1..1024;


    println!("Scanning ports on {target}...");

    for port in port_range {
            let addr = format!("{}:{}", target, port);

            match TcpStream::connect(&addr).await {
                Ok(_) => println!("Port {} is open", port),
                Err(_) => {} //Will remain silent if port is closed
            }
    }   

    println!("Scan is complete")


}