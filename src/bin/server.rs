use core::str;
use rand::seq::SliceRandom;
use std::{io, net::SocketAddr};
use tokio::net::UdpSocket;

static SERVER_MESSAGES: [&str; 6] = [
    "Hello, client!",
    "How's it going client?",
    "Ping!",
    "Rust is awesome!",
    "I love Rust!",
    "UDP rocks!",
];

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();

    let addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    let sock = UdpSocket::bind(addr).await?;
    println!("Server running on {}", sock.local_addr()?);

    let mut buf = [0; 1024];

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        println!(
            "Server received: {:?}",
            str::from_utf8(&buf[..len]).unwrap()
        );

        let msg = SERVER_MESSAGES.choose(&mut rng).unwrap();

        let len = sock.send_to(msg.as_bytes(), addr).await?;
        println!("{:?} bytes sent to {:?}", len, addr);
    }
}
