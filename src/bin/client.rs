use core::str;
use std::{io, net::SocketAddr};
use tokio::{
    net::UdpSocket,
    time::{Duration, sleep},
};

use rusty_moves::{
    GameAndPlayer, Message,
    tictactoe::{
        TTTGameResult, TTTGameState, TTTPlayer, pretty_print_board, tictactoe_rand,
        ttt_get_game_status,
    },
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

    // Start off with new game
    // Server plays first move, client chooses side
    let mut player = TTTPlayer::Circle;

    let mut win_count = 0;
    let mut loss_count = 0;
    let mut draw_count = 0;

    let msg = Message::NewGame(GameAndPlayer::TicTacToe(player));
    let str = msg.to_string();

    let len = sock.send(str.as_bytes()).await?;
    println!("Sent: {} bytes", len);

    loop {
        let len = sock.recv(&mut buf[..]).await?;
        let str = str::from_utf8(&buf[..len]).unwrap();
        //println!("Received: {} bytes", len);

        sleep(Duration::from_millis(5)).await;

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
                let len = sock.send(str.as_bytes()).await?;

                pretty_print_board(&str);
                println!("Move: {:?}\nSent: {} bytes", chosen_move, len);
            }
            Message::GameMsg(board) => {
                let game_state = TTTGameState::try_from(board).expect("Game invalid");
                let (chosen_move, msg) = tictactoe_rand(game_state, &player);

                let str = msg.to_string();
                let len = sock.send(str.as_bytes()).await?;

                pretty_print_board(&str);
                println!("Move: {:?}\nSent: {} bytes", chosen_move, len);

                if let Message::GameOver(_, res) = &msg {
                    if res == "draw" {
                        draw_count += 1;
                    } else {
                        win_count += 1;
                    }
                    println!(
                        "Client Stats: {} W | {} D | {} L",
                        win_count, draw_count, loss_count
                    );
                    if win_count + draw_count + loss_count >= 1000 {
                        break;
                    }
                    sleep(Duration::from_millis(50)).await;
                }
            }
            Message::GameOver(board, server_result) => {
                let game_state = TTTGameState::try_from(board).expect("Game invalid");
                if let Some(client_result) = ttt_get_game_status(&game_state, None) {
                    if client_result.to_string() == server_result {
                        match client_result {
                            TTTGameResult::Draw => {
                                println!("Draw acknowledged by client.");
                                draw_count += 1;
                            }
                            _ => {
                                println!("Win acknowledged by client.");
                                loss_count += 1;
                            }
                        };

                        println!(
                            "Client Stats: {} W | {} D | {} L",
                            win_count, draw_count, loss_count
                        );
                        if win_count + draw_count + loss_count >= 1000 {
                            break;
                        }

                        println!("New Game");

                        sleep(Duration::from_millis(100)).await;

                        player = TTTPlayer::Circle;
                        let msg = Message::NewGame(GameAndPlayer::TicTacToe(player));
                        let str = msg.to_string();

                        let len = sock.send(str.as_bytes()).await?;
                        println!("Sent: {} bytes", len);
                    } else {
                        println!(
                            "Error: Result mismatch!\nServer: {}\nClient: {}\nBoard: {}",
                            server_result, client_result, game_state
                        );
                    }
                } else {
                    println!(
                        "Error: Result mismatch!\nServer: {}\nClient: Game not finished.\nBoard: {}",
                        server_result, game_state
                    );
                }
            }
            Message::NewGame(GameAndPlayer::Chess(_)) => todo!(),
        }
    }

    Ok(())
}
