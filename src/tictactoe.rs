use anyhow::anyhow;
use rand::seq::IndexedRandom;
use std::fmt;

use crate::Message;

// Implemented for possible extensibility
const BOARD_SIZE: usize = 20;
const WIN_CONDITION: usize = 10;

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
        if str.len() != (BOARD_SIZE * BOARD_SIZE) {
            return Err(anyhow!(
                "String length incorrect: Expected {}, Actual {}",
                BOARD_SIZE * BOARD_SIZE,
                str.len()
            ));
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

pub fn pretty_print_board(msg: &str) {
    let extra_len = msg.len() - BOARD_SIZE * BOARD_SIZE; // "win"/"draw"
    if extra_len > 0 {
        println!("{}", &msg[0..extra_len]);
    }

    for row in 0..BOARD_SIZE {
        let start = extra_len + row * BOARD_SIZE;
        let end = start + BOARD_SIZE;
        println!("{}", &msg[start..end]);
    }
}

fn check_line(
    game_state: &TTTGameState,
    current: &TTTBlockState,
    start_x: isize,
    start_y: isize,
    direction: (isize, isize),
) -> bool {
    let (mut x, mut y) = (start_x, start_y);
    let mut acc: u32 = 0;

    for t in 0..BOARD_SIZE {
        if x < 0 || x >= BOARD_SIZE as isize || y < 0 || y >= BOARD_SIZE as isize {
            break;
        }

        if game_state.board[x as usize][y as usize] == *current {
            acc += 1;
            if acc == WIN_CONDITION as u32 {
                return true;
            }
        } else {
            acc = 0;
            // Max blocks left: BOARD_SIZE - t - 1
            if BOARD_SIZE - t - 1 < WIN_CONDITION {
                return false;
            }
        }

        x += direction.0;
        y += direction.1;
    }
    false
}

pub fn ttt_get_game_status(
    game_state: &TTTGameState,
    last_move: Option<(&usize, &usize)>,
) -> Option<TTTGameResult> {
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)]; // horizontal, vertical, two diagonals
    let board = &game_state.board;
    let mut winner: Option<TTTBlockState> = None;
    let mut is_draw = true;

    if let Some((px, py)) = last_move {
        // Last move known - only check relevant lines
        let player = &game_state.board[*px][*py];
        if check_line(game_state, player, *px as isize, 0, (0, 1)) {
            // Column
            winner = Some(*player);
        } else if check_line(game_state, player, 0, *py as isize, (1, 0)) {
            // Row
            winner = Some(*player);
        } else {
            // Diagonals
            let max_diag_len = BOARD_SIZE - px.abs_diff(*py);
            let diag_distance = px.min(py);

            let max_antidag_len =
                BOARD_SIZE - (*px as isize + *py as isize - BOARD_SIZE as isize + 1).unsigned_abs();
            let antidiag_distance = (BOARD_SIZE - px - 1).min(*py);

            if max_diag_len >= WIN_CONDITION
                && check_line(
                    game_state,
                    player,
                    (px - diag_distance) as isize,
                    (py - diag_distance) as isize,
                    (1, 1),
                )
            {
                // \ direction
                winner = Some(*player);
            } else if max_antidag_len >= WIN_CONDITION
                && check_line(
                    game_state,
                    player,
                    (px + antidiag_distance) as isize,
                    (py - antidiag_distance) as isize,
                    (-1, 1),
                )
            {
                // / direction
                winner = Some(*player);
            }
            // else: no wins
        }
    } else {
        // No last move - full board check
        'outer: for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if board[i][j] == TTTBlockState::Empty {
                    is_draw = false;
                    continue;
                }

                // Given board size n and win condition m,
                //   a m-in-a-row cannot start anywhere between
                //   (n-m+1, n-m+1) and (m-2, m-2)
                // Example: n=5, m=4, cannot start in (2, 2)
                #[allow(clippy::int_plus_one)]
                if i >= BOARD_SIZE - WIN_CONDITION + 1
                    && i <= WIN_CONDITION - 2
                    && j >= BOARD_SIZE - WIN_CONDITION + 1
                    && j <= WIN_CONDITION - 2
                {
                    continue;
                }

                let current = &board[i][j];
                for &(dx, dy) in &directions {
                    let mut failed = false; // Whether win condition fails
                    for step in 1..WIN_CONDITION as isize {
                        let (px, py) = (i as isize + dx * step, j as isize + dy * step);
                        if px < 0
                            || px >= BOARD_SIZE as isize
                            || py < 0
                            || py >= BOARD_SIZE as isize
                        {
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

    if let Some(result) = ttt_get_game_status(&game_state, Some((px, py))) {
        return (
            (*px, *py),
            Message::GameOver(game_state.to_string(), result.to_string()),
        );
    }

    if empty_blocks.len() == 1 {
        // Draw - last move made, since ttt_get_game_status above only checks changed lines and cannot detect draws
        return (
            (*px, *py),
            Message::GameOver(game_state.to_string(), TTTGameResult::Draw.to_string()),
        );
    }

    ((*px, *py), Message::GameMsg(game_state.to_string()))
}
