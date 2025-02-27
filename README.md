# Connect4 Match Generator & Analyzer

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)](https://github.com/TheGhoul21/connect-4-gen/actions)

This Rust project simulates Connect4 matches, analyzes the moves, and serializes the results to various formats (JSON, JSON Lite, Compact). It uses `rayon` for parallel match generation, `rand` for random move selection, and `serde` for serialization. The code also includes functionality to replay and visualize a specific match from the generated data, and provides a command-line interface for configuration.

## Features

*   **Parallel Match Generation:** Generates a specified number of Connect4 matches in parallel using Rayon, significantly speeding up the process.
*   **Random Move Selection:** Each player in a match selects a valid column randomly.
*   **Immediate Win Detection:** Before each move, the code checks if the current player has any immediate winning moves available. This information is recorded in the match data.
*   **Move Recording:** Each move is recorded, including the chosen column, the player, and whether immediate win positions were available.
*   **Multiple Output Formats:** Supports outputting match data in JSON (full), JSON Lite (condensed), and Compact (binary) formats.
*   **JSON Serialization:** The generated match data can be serialized to a JSON file for easy storage and later analysis.
*   **Match Replay & Visualization:** A function is provided to reconstruct the board state after each move of a specific match and print it to the console using Unicode circles.
*   **Clear Board Representation:** Uses Unicode circles (🟡, 🔴, ⚪) to visually represent the board state.
*   **Command-Line Interface (CLI):**  Provides a CLI for configuring the number of matches, output format, and other options.
*   **Interactive Mode:**  Allows configuring the application through an interactive prompt.
*   **Parsing Mode:**  Enables parsing an existing match file and displaying a specific match.
*   **Modularity:** Functions have been created to improve code organization and readability.

## Dependencies

This project relies on the following crates:

*   [`rand`](https://crates.io/crates/rand): For random number generation.
*   [`rayon`](https://crates.io/crates/rayon): For parallel processing.
*   [`serde`](https://crates.io/crates/serde): For serialization and deserialization (with `derive` feature).
*   [`serde_json`](https://crates.io/crates/serde_json): For JSON serialization.

These dependencies are declared in the `Cargo.toml` file and will be automatically downloaded and built by Cargo.

## Building and Running

1.  **Prerequisites:** Make sure you have Rust and Cargo installed. You can install them from [the official Rust website](https://www.rust-lang.org/tools/install).

2.  **Clone the Repository:**

    ```bash
    git clone https://github.com/TheGhoul21/connect-4-gen.git
    cd connect-4-gen
    ```

3.  **Build the Project:**

    ```bash
    cargo build --release
    ```

    This command compiles the project in release mode, creating an optimized executable in the `target/release` directory.

4.  **Run the Project:**

    You can run the project with various options using the command-line interface.  Here are some examples:

    *   **Generate matches with default settings:**

        ```bash
        cargo run --release
        ```

        This will generate 1000 matches in `jsonlite` format and save them to `matches_lite.json`.

    *   **Generate 5000 matches in JSON format and save them to `my_matches.json`:**

        ```bash
        cargo run --release -- -n 5000 -f json -o my_matches.json
        ```

    *   **Run in interactive mode:**

        ```bash
        cargo run --release -- -i
        ```

        This will guide you through a series of prompts to configure the match generation.

    *   **Parse an existing file and display match with ID 37:**

        ```bash
        cargo run --release -- parse --in matches.json --id 37
        ```

        This will parse the `matches.json` file and print the moves of the match with ID 37 to the console.

## Command-Line Interface (CLI)

The application supports the following command-line options:

*   `gen`:  (Default) Generate matches.
*   `parse`: Parse an already generated file and print a given board.
*   `-h`, `--help`: Show the help message.
*   `-n`, `--num-matches <NUM>`: Number of matches to simulate (default: 1000).
*   `-f`, `--format <FORMAT>`: Output format: `json`, `jsonlite`, `compact` (default: `jsonlite`).
*   `-w`, `--store-immediate-wins`: Store immediate win statistics (default: `true`).
*   `-o`, `--output <FILE>`: Output file (default: `matches.json` or `matches_lite.json` or `matches.bin`).
*   `-i`, `--interactive`: Run in interactive mode.
*   `-in`, `--input <FILE>`: Parses an already generated file (Mandatory field in `parse` mode).
*   `-id`, `--id <ID>`: The ID of the match to show (Mandatory field in `parse` mode).

**Examples:**

*   `connect-4-gen -n 5000 -f json -o my_matches.json`: Generates 5000 matches in JSON format and saves them to `my_matches.json`.
*   `connect-4-gen --interactive`: Runs in interactive mode.
*   `connect-4-gen parse --in matches.json --id 37`: Parses `matches.json` and displays match ID 37.

## Code Structure

*   **`Player` Enum:** Represents the two players (Yellow and Red).
*   **`Board` Struct:** Represents the Connect4 board. Includes methods for:
    *   `new()`: Creates a new, empty board.
    *   `can_play(col)`: Checks if a move in the given column is valid.
    *   `play(col, player)`: Attempts to place a piece in the given column. Returns the row/col position if successful, or `None` if the move is invalid.
    *   `is_winning_move(row, col, player)`: Checks if the move at (row, col) resulted in a win for the given player.
    *   `immediate_wins(player)`: Checks if the current player has any immediate winning moves available.
*   **`MoveRecord` Struct:** Stores information about a single move:
    *   `usr_move`: The column chosen by the player.
    *   `has_immediate_win`: Whether there was at least one winning move in the previous turn.
    *   `immediate_win_positions`: Positions of immediate win.
    *   `player`: The player who made the move.
*   **`Match` Struct:** Represents a Connect4 match, containing a vector of `MoveRecord`s and an ID.
*   **`random_connect4_match(rng)`:** Simulates a single Connect4 match, returning a vector of `MoveRecord`s.
*   **`print_board(board)`:** Prints the current state of the board to the console.
*   **`print_match_moves(moves)`:** Replays and prints the board after each move in a given match.
*   **`main()`:**
    *   Parses command-line arguments.
    *   Generates the specified number of matches in parallel.
    *   Serializes the results to the specified format.
    *   Optionally replays and prints a chosen match.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! If you find a bug or have a feature suggestion, please open an issue or submit a pull request. Before submitting a pull request, please ensure your code is formatted correctly (using `cargo fmt`) and that all tests pass (using `cargo test`).

## Future Improvements

*   Add more unit tests.
*   Implement a more sophisticated AI (beyond random moves).
*   Create more advanced analysis of matches.
*   Implement the compact output format.
*   Add more configuration options to the CLI.

[![Proudly generated by AI](https://img.shields.io/badge/Proudly%20Generated%20by-AI-success)](https://openai.com)
