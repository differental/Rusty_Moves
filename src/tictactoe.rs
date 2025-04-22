use anyhow::anyhow;
use rand::seq::IndexedRandom;
use std::fmt;

use crate::Message;

// Implemented for possible extensibility
static BOARD_SIZE: usize = 5;
static WIN_CONDITION: isize = 5;

#[derive(Clone, Copy, PartialEq)]
pub enum TTTBlockState {
    Empty,
    Circle,
    Cross,
}

impl TTTBlockState {
    fn to_char(self) -> char {
        match self {
            TTTBlockState::Empty => ' ',
            TTTBlockState::Circle => 'o',
            TTTBlockState::Cross => 'x',
        }
    }
}

#[derive(Clone, Copy)]
pub enum TTTPlayer {
    Circle,
    Cross,
}

impl fmt::Display for TTTPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            TTTPlayer::Circle => "o",
            TTTPlayer::Cross => "x",
        };
        write!(f, "{}", str)
    }
}

#[derive(PartialEq)]
pub enum TTTGameResult {
    Draw,
    CircleWin,
    CrossWin,
}

impl fmt::Display for TTTGameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            TTTGameResult::Draw => "draw",
            _ => "win", // Winner must be last player
        };
        write!(f, "{}", str)
    }
}

pub struct TTTGameState {
    board: [[TTTBlockState; BOARD_SIZE]; BOARD_SIZE],
}

impl TTTGameState {
    pub fn new() -> TTTGameState {
        TTTGameState {
            board: [[TTTBlockState::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

impl Default for TTTGameState {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<String> for TTTGameState {
    type Error = anyhow::Error;
    fn try_from(str: String) -> Result<Self, Self::Error> {
        if str.len() != (BOARD_SIZE * BOARD_SIZE) as usize {
            return Err(anyhow!("String length is not 9"));
        }

        let blocks = str
            .chars()
            .map(|x| match x {
                ' ' => Ok(TTTBlockState::Empty),
                'o' => Ok(TTTBlockState::Circle),
                'x' => Ok(TTTBlockState::Cross),
                _ => Err(anyhow!("Invalid character in input: '{}'", x)),
            })
            .collect::<anyhow::Result<Vec<TTTBlockState>>>()?;

        let mut board = [[TTTBlockState::Empty; BOARD_SIZE]; BOARD_SIZE];

        for i in 0..BOARD_SIZE {
            for j in 0usize..BOARD_SIZE {
                board[i][j] = blocks[BOARD_SIZE * i + j];
            }
        }

        Ok(TTTGameState { board })
    }
}

impl fmt::Display for TTTGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = self
            .board
            .as_flattened()
            .iter()
            .map(|x| x.to_char())
            .collect::<String>();
        write!(f, "{}", str)
    }
}

pub fn pretty_print_board(board: &str) {
    for row in 0..BOARD_SIZE {
        let start = row * 5;
        let end = start + 5;
        println!("{}", &board[start..end]);
    }
}

pub fn ttt_get_game_status(game_state: &TTTGameState) -> Option<TTTGameResult> {
    let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)]; // horizontal, vertical, two diagonals
    let board = &game_state.board;
    let mut winner: Option<TTTBlockState> = None;
    let mut is_draw = true;

    'outer: for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board[i as usize][j as usize] == TTTBlockState::Empty {
                is_draw = false;
                continue;
            }

            // Given board size n and win condition m,
            //   a m-in-a-row cannot start anywhere between
            //   (n-m+1, n-m+1) and (m-2, m-2)
            // Example: n=5, m=4, cannot start in (2, 2)
            if i >= BOARD_SIZE - WIN_CONDITION as usize + 1
                && i <= WIN_CONDITION as usize - 2
                && j >= BOARD_SIZE - WIN_CONDITION as usize + 1
                && j <= WIN_CONDITION as usize - 2
            {
                continue;
            }

            let current = &board[i as usize][j as usize];
            for &(dx, dy) in &directions {
                let mut failed = false; // Whether win condition fails
                for step in 1..WIN_CONDITION {
                    let (px, py) = (
                        i as isize + dx as isize * step,
                        j as isize + dy as isize * step,
                    );
                    if px < 0 || px >= BOARD_SIZE as isize || py < 0 || py >= BOARD_SIZE as isize {
                        failed = true;
                        break;
                    }
                    if &board[px as usize][py as usize] != current {
                        failed = true;
                        break;
                    }
                }
                if !failed {
                    is_draw = false;
                    winner = Some(*current);
                    break 'outer;
                }
            }
        }
    }

    if is_draw {
        return Some(TTTGameResult::Draw);
    }

    winner.map(|x| match x {
        TTTBlockState::Circle => TTTGameResult::CircleWin,
        TTTBlockState::Cross => TTTGameResult::CrossWin,
        TTTBlockState::Empty => unreachable!(),
    })
}

#[cfg(any())]
pub fn tictactoe_best(game_state: TTTGameState, player: TTTPlayer) {}

pub fn tictactoe_rand(
    mut game_state: TTTGameState,
    player: &TTTPlayer,
) -> ((usize, usize), Message) {
    let mut empty_blocks = vec![];

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if game_state.board[i][j] == TTTBlockState::Empty {
                empty_blocks.push((i, j));
            }
        }
    }

    let (px, py) = empty_blocks.choose(&mut rand::rng()).unwrap();
    game_state.board[*px][*py] = match player {
        TTTPlayer::Circle => TTTBlockState::Circle,
        TTTPlayer::Cross => TTTBlockState::Cross,
    };

    if let Some(result) = ttt_get_game_status(&game_state) {
        return (
            (*px, *py),
            Message::GameOver(game_state.to_string(), result.to_string()),
        );
    }

    ((*px, *py), Message::GameMsg(game_state.to_string()))
}
