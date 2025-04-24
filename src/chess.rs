use std::{fmt, io::{BufRead, BufReader, Write}, process::{Command, Stdio}};

use anyhow::anyhow;
use rand::seq::IndexedRandom;
use regex::Regex;

#[derive(Clone, Copy, PartialEq)]
pub enum ChessPlayer {
    White,
    Black,
}

impl fmt::Display for ChessPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            ChessPlayer::White => "w",
            ChessPlayer::Black => "b",
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Evaluation {
    CentiPawn(i32), // Centipawn difference (-ve: current player worse by)
    MateIn(i32) // Mate in N (-ve: current player getting mated)
}


#[derive(Debug)]
pub struct ChessGameState {
    fen: String,
    eval: Evaluation
}

impl ChessGameState {
    pub fn new() -> ChessGameState {
        ChessGameState {
            fen: String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            eval: Evaluation::CentiPawn(7)
        }
    }
}

/// Helper function to get top moves from stockfish
/// Returns Vec<(evaluation, move)>
pub fn get_stockfish_moves(player:&ChessPlayer, game_state: &ChessGameState, top_n: u32) -> Vec<(Evaluation, String)> {
    let depth = 10;
    let prefix = format!("info depth {}", depth);

    // stockfish must be in PATH
    let mut child = Command::new("stockfish")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start stockfish");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout);

    // writing to stdin sends uci commands to stockfish
    // Exchanges required: 
    //   1. uci - uciok;
    //   2. setoption... isready - readyok
    //   3. position... isready - readyok
    //   4. go depth 20 - {read result}

    let mut line = String::new(); // Reused as buffer
    
    // uci
    writeln!(stdin, "uci").unwrap();
    stdin.flush().unwrap();

    // check for uciok
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.trim() == "uciok" {
            line.clear();
            break;
        }
        line.clear();
    }

    // setoption + isready
    writeln!(stdin, "setoption name MultiPV value {top_n}\nisready").unwrap();
    stdin.flush().unwrap();

    // check for readyok
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.trim() == "readyok" {
            line.clear();
            break;
        }
        line.clear();
    }

    // set position (test) + isready
    writeln!(stdin, "position fen {}\nisready", game_state.fen).unwrap();
    stdin.flush().unwrap();

    // check for readyok
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.trim() == "readyok" {
            line.clear();
            break;
        }
        line.clear();
    }

    // go depth 20
    writeln!(stdin, "go depth {depth}").unwrap();
    stdin.flush().unwrap();

    let mut outputs = Vec::<String>::new();
    let mut line = String::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        if line.starts_with(&prefix) {
            outputs.push(line.trim().to_string());
        }
        if line.starts_with("bestmove") {
            break;
        }
        line.clear();
    }

    let re = Regex::new(r"multipv (.*) score (cp .*|mate .*) nodes .* pv ([a-h][1-8][a-h][1-8][q|r|n|b]?)").unwrap();

    // Top fives moves, in (move no.)
    let mut results = Vec::new();

    for str in outputs {
        if let Some(caps) = re.captures(&str) {
            // Note: caps[0] is the overall match (the entire string)
            //   [n] corresponds to $n where n is an integer

            // caps[1] is currently ignored - information should come ordered
            // caps[2] can be "cp %d" or "mate %d"
            if caps[2].starts_with("cp ") {
                let mut centipawn = caps[2][3..].parse::<i32>().unwrap();
                if player == &ChessPlayer::Black {
                    centipawn *= -1;
                }
                results.push((Evaluation::CentiPawn(centipawn), caps[3].to_string()));
            } else if caps[2].starts_with("mate ") {
                let mut mate_in = caps[2][3..].parse::<i32>().unwrap();
                if player == &ChessPlayer::Black {
                    mate_in *= -1;
                }
                results.push((Evaluation::MateIn(mate_in), caps[3].to_string()));
            }
        }
    }

    results
}

/// Chooses a random move from top five stockfish moves (might be less than that in certain cases)
/// player: Black or White. Determines only the evaluation in the returned game state
/// game_state: fen and evaluation after the last move
pub fn random_stockfish_move(player: ChessPlayer, game_state: ChessGameState) -> anyhow::Result<(String, ChessGameState)> {
    let best_moves = get_stockfish_moves(&player, &game_state, 5);
    let chosen_move = best_moves.choose(&mut rand::rng()).unwrap();

    // stockfish must be in PATH
    let mut child = Command::new("stockfish")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start stockfish");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout);

    let mut line = String::new(); // Reused as buffer
    
    // uci
    writeln!(stdin, "uci").unwrap();
    stdin.flush().unwrap();

    // check for uciok
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.trim() == "uciok" {
            line.clear();
            break;
        }
        line.clear();
    }

    // set pos and new move + isready
    writeln!(stdin, "position fen {} moves {}\nisready", game_state.fen, chosen_move.1).unwrap();
    stdin.flush().unwrap();

    // check for readyok
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.trim() == "readyok" {
            line.clear();
            break;
        }
        line.clear();
    }

    // get fen of current position after the chosen move
    writeln!(stdin, "d").unwrap();
    stdin.flush().unwrap();

    let mut line = String::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        if line.trim().starts_with("Fen: ") {
            return Ok((chosen_move.1.clone(), ChessGameState{ fen: line.trim()[5..].to_string(), eval: chosen_move.0 }));
        }
        line.clear();
    }
    Err(anyhow!("No Fen found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stockfish() {
        let data = random_stockfish_move(ChessPlayer::White, ChessGameState::new()).unwrap();
        println!("{:#?}", data);
    }
}

