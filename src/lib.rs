pub mod chess;
pub mod tictactoe;

use std::fmt;

use chess::ChessPlayer;
use tictactoe::TTTPlayer;

pub enum GameAndPlayer {
    Chess(ChessPlayer),
    TicTacToe(TTTPlayer),
}

pub enum Message {
    NewGame(GameAndPlayer),
    GameMsg(String),
    GameOver(String, String),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewGame(GameAndPlayer::TicTacToe(player)) => write!(f, "start:ttc,{}", player),
            Self::NewGame(GameAndPlayer::Chess(player)) => write!(f, "start:chess,{}", player),
            Self::GameMsg(board) => write!(f, "{}", board),
            Self::GameOver(board, result) => write!(f, "game-over:\n{}\n{}", result, board),
        }
    }
}

impl From<&str> for Message {
    fn from(str: &str) -> Self {
        match str {
            "start:ttc,o" => Self::NewGame(GameAndPlayer::TicTacToe(TTTPlayer::Circle)),
            "start:ttc,x" => Self::NewGame(GameAndPlayer::TicTacToe(TTTPlayer::Cross)),
            "start:chess,w" => Self::NewGame(GameAndPlayer::Chess(ChessPlayer::White)),
            "start:chess,b" => Self::NewGame(GameAndPlayer::Chess(ChessPlayer::Black)),
            str if str.starts_with("game-over") => {
                let mut lines = str.split('\n').map(String::from).collect::<Vec<String>>();
                let result = lines.remove(1);
                let board = lines.remove(1); // Becomes new line 1
                Self::GameOver(board, result)
            }
            str => Self::GameMsg(str.to_string()),
        }
    }
}
