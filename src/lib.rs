pub mod chess;
pub mod tictactoe;

use std::fmt;

use chess::ChessPlayer;
use tictactoe::TTTPlayer;

pub enum GameAndPlayer {
    Chess(ChessPlayer),
    TicTacToe(TTTPlayer),
}

pub enum Game {
    Chess,
    TicTacToe
}

pub enum Message {
    NewGame(GameAndPlayer),
    GameMsg(Game, String),
    GameOver(Game, String, String),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewGame(GameAndPlayer::TicTacToe(player)) => write!(f, "start:ttt,{}", player),
            Self::NewGame(GameAndPlayer::Chess(player)) => write!(f, "start:chess,{}", player),
            Self::GameMsg(Game::TicTacToe, board) => write!(f, "ttt:{}", board),
            Self::GameMsg(Game::Chess, board) => write!(f, "chess:{}", board),
            Self::GameOver(Game::TicTacToe, board, result) => write!(f, "ttt-game-over:\n{}\n{}", result, board),
            Self::GameOver(Game::Chess, board, result) => write!(f, "chess-game-over:\n{}\n{}", result, board)
        }
    }
}

impl From<&str> for Message {
    fn from(str: &str) -> Self {
        match str {
            "start:ttt,o" => Self::NewGame(GameAndPlayer::TicTacToe(TTTPlayer::Circle)),
            "start:ttt,x" => Self::NewGame(GameAndPlayer::TicTacToe(TTTPlayer::Cross)),
            "start:chess,w" => Self::NewGame(GameAndPlayer::Chess(ChessPlayer::White)),
            "start:chess,b" => Self::NewGame(GameAndPlayer::Chess(ChessPlayer::Black)),
            str if str.starts_with("ttt-game-over") => {
                let mut lines = str.split('\n').map(String::from).collect::<Vec<String>>();
                let result = lines.remove(1);
                let board = lines.remove(1); // Becomes new line 1
                Self::GameOver(Game::TicTacToe, board, result)
            },
            str if str.starts_with("chess-game-over") => {
                todo!();
                let mut lines = str.split('\n').map(String::from).collect::<Vec<String>>();
                let result = lines.remove(1);
                let board = lines.remove(1); // Becomes new line 1
                Self::GameOver(Game::Chess, board, result)
            }
            str if str.starts_with("ttt:") => {
                let mut lines = str.split("ttt:").map(String::from).collect::<Vec<String>>();
                let msg = lines.remove(1);
                Self::GameMsg(Game::TicTacToe, msg)
            }
            str if str.starts_with("chess:") => {
                let mut lines = str.split("chess:").map(String::from).collect::<Vec<String>>();
                let msg = lines.remove(1);
                Self::GameMsg(Game::Chess, msg)
            },
            _ => unreachable!()
        }
    }
}
