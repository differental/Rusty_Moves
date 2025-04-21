use core::str;
use rand::seq::SliceRandom;
use std::{io, net::SocketAddr};
use tokio::{
    net::UdpSocket,
    time::{Duration, Instant, sleep},
};

static CLIENT_MESSAGES: [&str; 6] = [
    "Hello, server!",
    "How's it going server?",
    "Ping!",
    "Rust is awesome!",
    "I love Rust!",
    "UDP rocks!",
];

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();

    // Allow system to allocate a free port
    let client_addr = "0.0.0.0:0".parse::<SocketAddr>().unwrap();
    let sock = UdpSocket::bind(client_addr).await?;

    println!("Client running on {}", sock.local_addr()?);

    let server_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    sock.connect(server_addr).await?; // Sets default address of recv and send

    let mut buf = [0; 1024];

    loop {
        let msg = CLIENT_MESSAGES.choose(&mut rng).unwrap();

        let start = Instant::now();

        let len = sock.send(msg.as_bytes()).await?;
        println!("{:?} bytes sent to server", len);

        let len = sock.recv(&mut buf[..]).await?;
        println!("{:?} bytes received from server", len);

        println!("{:?} elapsed", start.elapsed());

        println!(
            "Client received: {:?}",
            str::from_utf8(&buf[..len]).unwrap()
        );

        sleep(Duration::from_millis(100)).await;
    }
}
