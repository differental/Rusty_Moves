use std::net::SocketAddr;
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    net::UdpSocket,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Allow system to allocate a free port
    let client_addr = "0.0.0.0:0".parse::<SocketAddr>().unwrap();
    let sock = UdpSocket::bind(client_addr).await?;

    println!("Client running on {}", sock.local_addr()?);

    let server_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    sock.connect(server_addr).await?; // Sets default address of recv and send

    let mut buf = [0; 1024];

    // Prepare to read lines from stdin
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    while let Some(line) = lines.next_line().await? {
        let msg = line.trim();
        if msg.is_empty() {
            continue; // skip empty lines
        }

        // send to server
        let sent: usize = sock.send(msg.as_bytes()).await?;
        println!("Sent ({} bytes): {}", sent, msg);

        // wait for a response
        let n = sock.recv(&mut buf).await?;
        let resp = String::from_utf8_lossy(&buf[..n]);
        println!("Received: {}", resp);
    }

    Ok(())
}