use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Player {
    Yellow,
    Red,
}

#[derive(Clone, Debug)]
struct Board {
    // 6 rows, 7 columns
    grid: [[Option<Player>; 7]; 6],
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [[None; 7]; 6],
        }
    }

    /// Return true if placing a piece in `col` is valid (i.e., not full).
    fn can_play(&self, col: usize) -> bool {
        // If top cell is not occupied, we can play.
        self.grid[0][col].is_none()
    }

    /// Attempt to place a piece for `player` in the given `col`.
    /// Returns (row, col) where it landed if successful, or None if invalid move.
    fn play(&mut self, col: usize, player: Player) -> Option<(usize, usize)> {
        if !self.can_play(col) {
            return None;
        }
        // Start from bottom row, move up until we find an empty space
        for row in (0..6).rev() {
            if self.grid[row][col].is_none() {
                self.grid[row][col] = Some(player);
                return Some((row, col));
            }
        }
        None
    }

    /// Check if the last move by `player` at (row, col) caused that player to win.
    fn is_winning_move(&self, row: usize, col: usize, player: Player) -> bool {
        // 1) Horizontal check
        let mut count = 1;
        // count left
        let mut c = col as i32 - 1;
        while c >= 0 && self.grid[row][c as usize] == Some(player) {
            count += 1;
            c -= 1;
        }
        // count right
        c = col as i32 + 1;
        while c < 7 && self.grid[row][c as usize] == Some(player) {
            count += 1;
            c += 1;
        }
        if count >= 4 {
            return true;
        }

        // 2) Vertical check
        count = 1;
        // count down
        let mut r = row as i32 + 1;
        while r < 6 && self.grid[r as usize][col] == Some(player) {
            count += 1;
            r += 1;
        }
        if count >= 4 {
            return true;
        }

        // 3) Diagonal 1 (\) check
        count = 1;
        // up-left
        let (mut r, mut c) = (row as i32 - 1, col as i32 - 1);
        while r >= 0 && c >= 0 && self.grid[r as usize][c as usize] == Some(player) {
            count += 1;
            r -= 1;
            c -= 1;
        }
        // down-right
        let (mut r, mut c) = (row as i32 + 1, col as i32 + 1);
        while r < 6 && c < 7 && self.grid[r as usize][c as usize] == Some(player) {
            count += 1;
            r += 1;
            c += 1;
        }
        if count >= 4 {
            return true;
        }

        // 4) Diagonal 2 (/) check
        count = 1;
        // up-right
        let (mut r, mut c) = (row as i32 - 1, col as i32 + 1);
        while r >= 0 && c < 7 && self.grid[r as usize][c as usize] == Some(player) {
            count += 1;
            r -= 1;
            c += 1;
        }
        // down-left
        let (mut r, mut c) = (row as i32 + 1, col as i32 - 1);
        while r < 6 && c >= 0 && self.grid[r as usize][c as usize] == Some(player) {
            count += 1;
            r += 1;
            c -= 1;
        }
        if count >= 4 {
            return true;
        }

        false
    }

    /// Check if the current player has any *immediate winning moves* available.
    /// Returns (has_immediate_win, immediate_win_positions).
    fn immediate_wins(&self, player: Player) -> (bool, Vec<(usize, usize)>) {
        let mut immediate_win_positions = Vec::new();
        // For each col that is playable, see if that move would immediately win.
        for col in 0..7 {
            if self.can_play(col) {
                // Temporarily drop piece
                let mut temp = self.clone();
                if let Some((row, col)) = temp.play(col, player) {
                    if temp.is_winning_move(row, col, player) {
                        immediate_win_positions.push((row, col)); 
                    }
                }
            }
        }

        ( !immediate_win_positions.is_empty(), immediate_win_positions )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MoveRecord {
    usr_move: usize,                // Which column was chosen
    has_immediate_win: bool,        // Whether the current position had at least one winning move
    immediate_win_positions: Vec<(usize, usize)>, // All winning row/col positions
    player: Player,                 // Which player made the move
}

fn random_connect4_match<R: Rng + ?Sized>(rng: &mut R) -> Vec<MoveRecord> {
    let mut board = Board::new();
    let mut moves: Vec<MoveRecord> = Vec::new();

    let mut current_player = Player::Yellow;

    loop {
        // Check if the current player has any *immediate* winning moves
        let (has_immediate_win, immediate_win_positions) = board.immediate_wins(current_player);

        // Collect valid columns
        let valid_cols: Vec<usize> = (0..7).filter(|&col| board.can_play(col)).collect();

        // If no valid moves remain (board full or no columns available), end the match
        if valid_cols.is_empty() {
            break;
        }

        // Pick a random valid column
        let chosen_index = rng.random_range(0..valid_cols.len());
        let col = valid_cols[chosen_index];

        // Place the piece
        let drop_pos = board.play(col, current_player).unwrap();

        // Record the move
        moves.push(MoveRecord {
            usr_move: col,
            has_immediate_win,
            immediate_win_positions,
            player: current_player,
        });

        // Check if this move won the game
        if board.is_winning_move(drop_pos.0, drop_pos.1, current_player) {
            // If the current player just won, stop the match.
            break;
        }

        // Switch players
        current_player = match current_player {
            Player::Yellow => Player::Red,
            Player::Red => Player::Yellow,
        };
    }

    moves
}

/// Pretty-print the board in its current state.
/// Using Unicode circles for demonstration.
fn print_board(board: &Board) {
    for row in 0..6 {
        print!("|");
        for col in 0..7 {
            match board.grid[row][col] {
                Some(Player::Yellow) => print!("🟡"),
                Some(Player::Red) => print!("🔴"),
                None => print!("⚪"),  // empty
            }
            print!("|");
        }
        println!();
    }
    println!("-----------------------------");
}

/// Given a sequence of moves, reconstruct and print the board after each move.
fn print_match_moves(moves: &[MoveRecord]) {
    let mut board = Board::new();
    for (i, m) in moves.iter().enumerate() {
        // Re-play the move on an empty board
        let col = m.usr_move;
        board.play(col, m.player);

        println!(
            "=== Move #{} by {:?} (has_immediate_win={}, positions={:?}) ===",
            i, m.player, m.has_immediate_win, m.immediate_win_positions
        );
        print_board(&board);
        println!();
    }
}

fn main() {
    let num_matches = 1000; // Generate N matches
    // 1) Generate matches in parallel
    //    (0..num_matches) is an iterator of match indices
    //    `.into_par_iter()` from Rayon makes it parallel
    let all_matches: Vec<Vec<MoveRecord>> = (0..num_matches)
        .into_par_iter()
        .map(|_i| {
            // Each thread uses its own RNG instance
            let mut rng = rand::rng();
            random_connect4_match(&mut rng)
        })
        .collect();

    // 2) Serialize them to JSON
    let output_file = std::fs::File::create("matches_lite.json")
        .expect("Failed to create JSON output file");
    serde_json::to_writer(output_file, &all_matches)
        .expect("Failed to write JSON");



    // 3) Now pick one match (by index) and print its board states
    let chosen_index = 2;
    if chosen_index < all_matches.len() {
        let chosen_match = &all_matches[chosen_index];
        println!("--- Now printing boards for match #{} ---", chosen_index);
        print_match_moves(chosen_match);
    } else {
        println!("No match at index {}", chosen_index);
    }
}
