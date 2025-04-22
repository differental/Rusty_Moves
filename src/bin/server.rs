use core::str;
use std::{io, net::SocketAddr, time::Duration};
use tokio::{net::UdpSocket, time::sleep};

use rusty_moves::{
    GameAndPlayer, Message,
    tictactoe::{
        TTTGameResult, TTTGameState, TTTPlayer, pretty_print_board, tictactoe_rand,
        ttt_get_game_status,
    },
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    let sock = UdpSocket::bind(addr).await?;
    println!("Server running on {}", sock.local_addr()?);

    let mut buf = [0; 1024];
    let mut player = TTTPlayer::Circle;

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let str = str::from_utf8(&buf[..len]).unwrap();
        //println!("[{}] Received: {} bytes", addr, len);

        sleep(Duration::from_millis(30)).await;

        let msg = Message::from(str);
        match msg {
            Message::NewGame(GameAndPlayer::TicTacToe(opponent)) => {
                player = match opponent {
                    TTTPlayer::Circle => TTTPlayer::Cross,
                    TTTPlayer::Cross => TTTPlayer::Circle,
                };
                let game_state = TTTGameState::new();
                let (chosen_move, msg) = tictactoe_rand(game_state, &player);

                let str = msg.to_string();
                let len = sock.send_to(str.as_bytes(), addr).await?;

                pretty_print_board(&str);
                println!("Move: {:?}\nSent: {} bytes", chosen_move, len);
            }
            Message::GameMsg(board) => {
                let game_state = TTTGameState::try_from(board).expect("Game invalid");
                let (chosen_move, msg) = tictactoe_rand(game_state, &player);

                let str = msg.to_string();
                let len = sock.send_to(str.as_bytes(), addr).await?;

                pretty_print_board(&str);
                println!("Move: {:?}\nSent: {} bytes", chosen_move, len);
            }
            Message::GameOver(board, client_result) => {
                let game_state = TTTGameState::try_from(board).expect("Game invalid");
                if let Some(server_result) = ttt_get_game_status(&game_state) {
                    if server_result.to_string() == client_result {
                        match server_result {
                            TTTGameResult::Draw => {
                                println!("Draw acknowledged by server.")
                            }
                            _ => println!("Win acknowledged by server."),
                        };

                        println!("New Game");

                        sleep(Duration::from_millis(10000)).await;

                        player = TTTPlayer::Circle;
                        let msg = Message::NewGame(GameAndPlayer::TicTacToe(player));
                        let str = msg.to_string();

                        let len = sock.send_to(str.as_bytes(), addr).await?;
                        println!("Sent: {} bytes", len);
                    } else {
                        println!(
                            "Error: Result mismatch!\nClient: {}\nServer: {}\nBoard: {}",
                            client_result, server_result, game_state
                        );
                    }
                } else {
                    println!(
                        "Error: Result mismatch!\nClient: {}\nServer: Game not finished.\nBoard: {}",
                        client_result, game_state
                    );
                }
            }
            Message::NewGame(GameAndPlayer::Chess(_)) => todo!(),
        }
    }
}
