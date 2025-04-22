use std::fmt;

#[derive(Clone, Copy)]
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

//pub fn get_best_moves(fen: &str, top_n: usize) -> Vec<(String, String)> {
// Send to stockfish
//}

/*
pub fn get_best_moves_chess() {
    let mut child = Command::new("stockfish") // Make sure Stockfish is in PATH
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start Stockfish");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout);

    // Send UCI command
    writeln!(stdin, "uci").unwrap();
    println!("Sending uci");
    stdin.flush().unwrap();

    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        println!("Stockfish: {}", line.trim());
        if line.trim() == "uciok" {
            break;
        }
        line.clear();
    }


    writeln!(stdin, "isready").unwrap();
    stdin.flush().unwrap();

    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        println!("Stockfish: {}", line.trim());
        if line.trim() == "readyok" {
            break;
        }
        line.clear();
    }

    let pv = 5;

    writeln!(stdin, "setoption name MultiPV value {pv}\nisready").unwrap();
    stdin.flush().unwrap();

    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        println!("Stockfish: {}", line.trim());
        if line.trim() == "readyok" {
            break;
        }
        line.clear();
    }


    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        println!("Stockfish: {}", line.trim());
        if line.trim() == "uciok" {
            break;
        }
        line.clear();
    }

    // Set position and ask for best move
    writeln!(stdin, "position startpos").unwrap();
    writeln!(stdin, "go depth 10").unwrap();
    stdin.flush().unwrap();

    loop {
        let mut output = String::new();
        if reader.read_line(&mut output).unwrap() == 0 {
            break;
        }
        println!("Stockfish: {}", output.trim());
        if output.starts_with("bestmove") {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        let result = 3; //get_best_moves(STARTING_POSITION, 2);

        println!("{:?}", result);
    }

    #[test]
    fn test_stockfish() {
        get_moves();
    }
}

*/
