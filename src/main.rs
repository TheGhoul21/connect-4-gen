use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;

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

/// Output formats supported by the CLI
#[derive(Debug, Clone, Copy, PartialEq)]
enum OutputFormat {
    Json,
    JsonLite,
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Match {
    moves: Vec<MoveRecord>,
    id: usize,
}

impl Match {
    pub fn new(id: usize, moves: Vec<MoveRecord>) -> Self {
        Self { id, moves }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "jsonlite" => Ok(OutputFormat::JsonLite),
            "compact" => Ok(OutputFormat::Compact),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::JsonLite => write!(f, "jsonlite"),
            OutputFormat::Compact => write!(f, "compact"),
        }
    }
}

enum ToolMode {
    Generation,
    Parsing,
}

struct AppConfig {
    mode: ToolMode,
    num_matches: usize,
    output_format: OutputFormat,
    store_immediate_wins: bool,
    output_file: Option<PathBuf>,
    input_file: Option<PathBuf>,
    id: Option<usize>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mode: ToolMode::Generation,
            num_matches: 1000,
            output_format: OutputFormat::JsonLite,
            store_immediate_wins: true,
            output_file: None,
            input_file: None,
            id: None,
        }
    }
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

        (!immediate_win_positions.is_empty(), immediate_win_positions)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoveRecord {
    usr_move: usize,                              // Which column was chosen
    has_immediate_win: bool, // Whether the current position had at least one winning move
    immediate_win_positions: Vec<(usize, usize)>, // All winning row/col positions
    player: Player,          // Which player made the move
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
                None => print!("⚪"), // empty
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

fn print_help() {
    println!("Connect-4 Match Generator");
    println!("");
    println!("USAGE:");
    println!("    connect-4-gen command [OPTIONS]");
    println!("");
    println!("COMMANDS:");
    println!("    gen   Default mode to generate matches");
    println!("    parse Parse an already generated file, and print a given board");
    println!("OPTIONS:");
    println!("    -h,   --help                     Show this help message");
    println!("    -n,   --num-matches <NUM>        Number of matches to simulate (default: 1000)");
    println!("    -f,   --format <FORMAT>          Output format: json, jsonlite, compact (default: jsonlite)");
    println!("    -w,   --store-immediate-wins     Store immediate win statistics (default: true)");
    println!("    -o,   --output <FILE>            Output file (default: matches.json or matches_lite.json)");
    println!("    -i,   --interactive              Run in interactive mode");
    println!("    -in,  --input <FILE>             Parses an already generated file (Mandatory field in parse mode)");
    println!("    -id,  --id <ID>                  THe ID of the match to show (Mandatory field in parse mode)");
    println!("");
    println!("EXAMPLES:");
    println!("    connect-4-gen -n 5000 -f json -o my_matches.json");
    println!("    connect-4-gen --interactive");
    println!("    connect-4-gen parse --in matches.json --id 37")
}

fn run_interactive_mode() -> AppConfig {
    let mut config = AppConfig::default();

    println!("=== Connect-4 Match Generator Interactive Mode ===");
    println!("Let's configure your match generation settings:");

    // Collect number of matches
    print!("Number of matches to simulate [1000]: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if !input.is_empty() {
        if let Ok(num) = input.parse::<usize>() {
            config.num_matches = num;
        } else {
            println!("Invalid input, using default: 1000");
        }
    }

    // Collect output format
    println!("\nOutput format options:");
    println!("  - json: Full JSON output including all move details");
    println!("  - jsonlite: Condensed JSON output (default)");
    println!("  - compact: Binary/compact representation");
    print!("Choose output format [jsonlite]: ");
    io::stdout().flush().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if !input.is_empty() {
        match input.parse::<OutputFormat>() {
            Ok(format) => config.output_format = format,
            Err(_) => println!("Invalid format, using default: jsonlite"),
        }
    }

    // Collect store_immediate_wins
    println!("\nStore immediate win statistics?");
    println!("This tracks whether players had winning moves available but didn't take them");
    print!("Store immediate win stats? (y/n) [y]: ");
    io::stdout().flush().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_lowercase();
    if !input.is_empty() {
        if input == "n" || input == "no" {
            config.store_immediate_wins = false;
        }
    }

    // Collect output file
    println!("\nOutput file (leave empty for default):");
    print!("Output file: ");
    io::stdout().flush().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if !input.is_empty() {
        config.output_file = Some(PathBuf::from(input));
    }

    // Summary
    println!("\nConfiguration summary:");
    println!("- Number of matches: {}", config.num_matches);
    println!("- Output format: {}", config.output_format);
    println!("- Store immediate wins: {}", config.store_immediate_wins);
    println!(
        "- Output file: {}",
        config
            .output_file
            .as_ref()
            .map_or("default".to_string(), |p| p.to_string_lossy().to_string())
    );

    config
}

fn parse_cli_args() -> AppConfig {
    let args: Vec<String> = std::env::args().collect();
    let mut config = AppConfig::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "parse" => {
                config.mode = ToolMode::Parsing;
            }
            "gen" => {
                config.mode = ToolMode::Generation;
            }
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-i" | "--interactive" => {
                return run_interactive_mode();
            }
            "-n" | "--num-matches" => {
                if i + 1 < args.len() {
                    if let Ok(num) = args[i + 1].parse() {
                        config.num_matches = num;
                    } else {
                        eprintln!("Error: Invalid number of matches");
                        std::process::exit(1);
                    }
                    i += 1;
                }
            }
            "-f" | "--format" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse() {
                        Ok(format) => config.output_format = format,
                        Err(_) => {
                            eprintln!("Error: Invalid output format");
                            std::process::exit(1);
                        }
                    }
                    i += 1;
                }
            }
            "-w" | "--store-immediate-wins" => {
                if i + 1 < args.len() {
                    match args[i + 1].to_lowercase().as_str() {
                        "true" | "yes" | "y" | "1" => config.store_immediate_wins = true,
                        "false" | "no" | "n" | "0" => config.store_immediate_wins = false,
                        _ => {
                            eprintln!("Error: Invalid value for store-immediate-wins (must be true/false)");
                            std::process::exit(1);
                        }
                    }
                    i += 1;
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    config.output_file = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "-in" | "--input" => {
                if i + 1 < args.len() {
                    config.input_file = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "-id" | "--id" => {
                if i + 1 < args.len() {
                    config.id = Some(usize::from_str(&args[i + 1]).unwrap());
                    i += 1;
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                eprintln!("Use --help for usage information");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    config
}

fn is_valid_config(config: &AppConfig) -> bool {
    match config.mode {
        ToolMode::Parsing => {
            if let None = config.input_file {
                panic!("Input file is mandatory in parse mode, add it with -in=path.json");
            }

            if let None = config.id {
                panic!("ID is mandatory in parse mode, add it with -id=<ID>");
            }

            true
        }
        _ => true,
    }
}

fn get_default_output_file(format: &OutputFormat) -> PathBuf {
    match format {
        OutputFormat::Json => PathBuf::from("matches.json"),
        OutputFormat::JsonLite => PathBuf::from("matches_lite.json"),
        OutputFormat::Compact => PathBuf::from("matches.bin"),
    }
}

fn do_generate(config: AppConfig) {
    println!("Generating {} matches...", config.num_matches);

    // Generate matches in parallel
    let all_matches: Vec<Match> = (0..config.num_matches)
        .into_par_iter()
        .map(|_i| {
            // Each thread uses its own RNG instance
            let mut rng = rand::rng();
            Match::new(_i + 1, random_connect4_match(&mut rng))
        })
        .collect();

    // Determine output file
    let output_path = config
        .output_file
        .unwrap_or_else(|| get_default_output_file(&config.output_format));

    // Process and save according to format
    match config.output_format {
        OutputFormat::Json | OutputFormat::JsonLite => {
            let output_file =
                std::fs::File::create(&output_path).expect("Failed to create output file");

            // If not storing immediate wins, filter them out
            if !config.store_immediate_wins {
                // Create a version with filtered data
                let filtered_matches: Vec<Vec<MoveRecord>> = all_matches
                    .iter()
                    .map(|match_moves| {
                        match_moves
                            .moves
                            .iter()
                            .map(|move_record| {
                                let mut filtered = move_record.clone();
                                filtered.immediate_win_positions = Vec::new();
                                filtered
                            })
                            .collect()
                    })
                    .collect();

                serde_json::to_writer(output_file, &filtered_matches)
                    .expect("Failed to write JSON output");
            } else {
                serde_json::to_writer(output_file, &all_matches)
                    .expect("Failed to write JSON output");
            }
        }
        OutputFormat::Compact => {
            // For a compact format, we could use a binary serialization format like bincode
            // This is a placeholder - implement actual compact format if needed
            eprintln!("Compact format not yet implemented");
            std::process::exit(1);
        }
    }

    println!(
        "Successfully generated {} matches and saved to {}",
        config.num_matches,
        output_path.display()
    );
}

fn do_parse(config: AppConfig) {
    if let Some(input_file) = &config.input_file {
        let file = std::fs::File::open(input_file).expect("Failed to open input file");
        let all_matches: Vec<Match> =
            serde_json::from_reader(file).expect("Failed to parse JSON");

        if let Some(id) = config.id {
            if id < all_matches.len() {
                let index = all_matches.iter().position(|match_moves| match_moves.id == id);
                if let None = index {
                    eprintln!("Error: Match ID {} not found", id);
                    std::process::exit(1);
                }
                let match_moves = &all_matches[index.unwrap()].moves;
                print_match_moves(match_moves);
            } else {
                eprintln!(
                    "Error: Match ID {} is out of range. Total matches: {}",
                    id,
                    all_matches.len()
                );
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: Match ID is required for parsing mode");
            std::process::exit(1);
        }
    } else {
        eprintln!("Error: Input file is required for parsing mode");
        std::process::exit(1);
    }
}

fn main() {
    // Parse command line arguments or run in interactive mode
    let config = parse_cli_args();

    // Check if the config is valid:
    if !is_valid_config(&config) {
        std::process::exit(1);
    }

    match config.mode {
        ToolMode::Generation => do_generate(config),
        ToolMode::Parsing => do_parse(config),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        for row in 0..6 {
            for col in 0..7 {
                assert!(board.grid[row][col].is_none());
            }
        }
    }

    #[test]
    fn test_can_play() {
        let mut board = Board::new();
        // All columns should be playable in a new board
        for col in 0..7 {
            assert!(board.can_play(col));
        }

        // Fill a column and check that it's no longer playable
        for _ in 0..6 {
            board.play(0, Player::Yellow);
        }
        assert!(!board.can_play(0));
    }

    #[test]
    fn test_play() {
        let mut board = Board::new();

        // First piece should land at the bottom row (row 5)
        let pos = board.play(3, Player::Yellow);
        assert_eq!(pos, Some((5, 3)));
        assert_eq!(board.grid[5][3], Some(Player::Yellow));

        // Second piece should stack on top (row 4)
        let pos = board.play(3, Player::Red);
        assert_eq!(pos, Some((4, 3)));
        assert_eq!(board.grid[4][3], Some(Player::Red));
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = Board::new();

        // Place 4 pieces in a row horizontally
        board.play(0, Player::Yellow);
        board.play(1, Player::Yellow);
        board.play(2, Player::Yellow);
        let pos = board.play(3, Player::Yellow).unwrap();

        assert!(board.is_winning_move(pos.0, pos.1, Player::Yellow));
    }

    #[test]
    fn test_vertical_win() {
        let mut board = Board::new();

        // Place 4 pieces in a column
        board.play(0, Player::Red);
        board.play(0, Player::Red);
        board.play(0, Player::Red);
        let pos = board.play(0, Player::Red).unwrap();

        assert!(board.is_winning_move(pos.0, pos.1, Player::Red));
    }

    #[test]
    fn test_diagonal_win() {
        let mut board = Board::new();

        // Create a diagonal win pattern
        // X
        // O X
        // O O X
        // O O O X
        board.play(0, Player::Yellow);
        board.play(0, Player::Yellow);
        board.play(0, Player::Yellow);
        board.play(0, Player::Red);

        board.play(1, Player::Yellow);
        board.play(1, Player::Yellow);
        board.play(1, Player::Red);

        board.play(2, Player::Yellow);
        board.play(2, Player::Red);

        let pos = board.play(3, Player::Red).unwrap();

        assert!(board.is_winning_move(pos.0, pos.1, Player::Red));
    }

    #[test]
    fn test_diagonal_reverse_win() {
        let mut board = Board::new();

        // Create a reverse diagonal win pattern (/)
        //       X
        //     X O
        //   X O O
        // X O O O
        board.play(6, Player::Yellow);
        board.play(6, Player::Yellow);
        board.play(6, Player::Yellow);
        board.play(6, Player::Red);

        board.play(5, Player::Yellow);
        board.play(5, Player::Yellow);
        board.play(5, Player::Red);

        board.play(4, Player::Yellow);
        board.play(4, Player::Red);

        let pos = board.play(3, Player::Red).unwrap();

        assert!(board.is_winning_move(pos.0, pos.1, Player::Red));
    }

    #[test]
    fn test_immediate_wins() {
        let mut board = Board::new();

        // Set up a board where Yellow has an immediate win
        // Place 3 Yellow pieces in a row
        board.play(0, Player::Yellow);
        board.play(1, Player::Yellow);
        board.play(2, Player::Yellow);

        // Check if Yellow has immediate wins
        let (has_win, positions) = board.immediate_wins(Player::Yellow);
        assert!(has_win);
        assert!(positions.contains(&(5, 3))); // Win at column 3
    }

    #[test]
    fn test_no_immediate_wins() {
        let mut board = Board::new();

        // Set up a board where there are no immediate wins
        board.play(0, Player::Yellow);
        board.play(2, Player::Yellow);
        board.play(4, Player::Yellow);

        // Check that there are no immediate wins
        let (has_win, positions) = board.immediate_wins(Player::Yellow);
        assert!(!has_win);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_random_match() {
        // Use a deterministic RNG for testing
        let mut rng = StepRng::new(42, 1);
        let moves = random_connect4_match(&mut rng);

        // Verify match terminates with a win or draw
        let mut board = Board::new();
        let mut last_pos = None;
        let mut last_player = Player::Yellow;

        for m in &moves {
            last_pos = board.play(m.usr_move, m.player);
            last_player = m.player;
        }

        if let Some((row, col)) = last_pos {
            // Either it's a win or the board is full
            let is_win = board.is_winning_move(row, col, last_player);

            if !is_win {
                // If not a win, board should be full or have no valid moves
                let valid_moves = (0..7).filter(|&col| board.can_play(col)).count();
                assert_eq!(valid_moves, 0);
            }
        }
    }

    #[test]
    fn test_invalid_play() {
        let mut board = Board::new();

        // Fill column 0
        for _ in 0..6 {
            board.play(0, Player::Yellow);
        }

        // Try to play in the full column
        let pos = board.play(0, Player::Red);
        assert_eq!(pos, None);
    }
}
