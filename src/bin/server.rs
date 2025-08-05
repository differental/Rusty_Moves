use std::str::from_utf8;
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use dashmap::DashMap;
use anyhow::{anyhow, Error};

enum GameState {
    //TicTacToe(TTTGameState),
    //Chess(ChessGameState),
    Counting(CountingGameState)
}

struct CountingGameState {
    start_val: i64,
    last_sent: i64
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    let sock = Arc::new(UdpSocket::bind(addr).await?);
    println!("Server running on {addr}");

    let state: Arc<DashMap<SocketAddr, GameState>> = Default::default();

    let mut buf = [0u8; 1024];

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let sock = Arc::clone(&sock);
        let state = Arc::clone(&state);

        let packet = buf[..len].to_vec(); // copy

        tokio::spawn(async move {
            match handle_packet(sock, state, addr, packet).await {
                Ok(()) => (),
                Err(err) => eprintln!("Error communicating with {addr}: {err}"),
            }
        });

    }

    Ok(())
}

async fn handle_packet(
    socket: Arc<UdpSocket>,
    state: Arc<DashMap<SocketAddr, GameState>>,
    addr: SocketAddr,
    packet: Vec<u8>
) -> anyhow::Result<()> {
    let msg = from_utf8(&packet)?.trim();
    // parse into game message here

    let mut start = 0;

    if msg.len() > 8 && &msg[0..8] == "ggstart:" {
        start = msg[8..].parse::<i64>()?;

        // DashMap doesn't require locking and works just like RwLock<HashMap<>> but faster.
        let reply = start + 1;

        state.insert(
            addr,
            GameState::Counting(CountingGameState { start_val: start, last_sent: reply })
        );
        
        socket.send_to(reply.to_string().as_bytes(), addr).await?;

    } else if msg.len() > 3 && &msg[0..3] == "gg:" {
        let response = msg[3..].parse::<i64>()?;

        // DashMap doesn't require locking and works just like RwLock<HashMap<>> but faster.
        let mut entry = match state.get_mut(&addr) {
            Some(entry) => entry,
            None => {
                socket.send_to("Active game not found.".as_bytes(), addr).await?;
                return Err(anyhow::anyhow!("No value found"));
            }
        };

        let game_state = entry.value_mut();
        if let GameState::Counting(counting_game_state) = game_state {
            let CountingGameState{ start_val: _, last_sent } = counting_game_state;
            if response == *last_sent + 1 {
                let reply = response + 1;
                *last_sent = reply;
                socket.send_to(reply.to_string().as_bytes(), addr).await?;
            } else {
                // Wrong answer!
                socket.send_to("Wrong number!".as_bytes(), addr).await?;
            }
        } else {
            let reply = response + 1;
            *game_state = GameState::Counting(CountingGameState { start_val: response, last_sent: reply });
            socket.send_to(reply.to_string().as_bytes(), addr).await?;
        }
    } else {
        socket.send_to("Not Supported".to_string().as_bytes(), addr).await?;
        return unimplemented!();
    }
    
    Ok(())
}
