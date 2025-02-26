# Connect4 Match Generator & Analyzer

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)](https://github.com/TheGhoul21/connect-4-gen/actions)  


This Rust project simulates Connect4 matches, analyzes the moves, and serializes the results to JSON.  It uses `rayon` for parallel match generation, `rand` for random move selection, and `serde` for serialization. The code also includes functionality to replay and visualize a specific match from the generated data.

## Features

*   **Parallel Match Generation:**  Generates a specified number of Connect4 matches in parallel using Rayon, significantly speeding up the process.
*   **Random Move Selection:**  Each player in a match selects a valid column randomly.
*   **Immediate Win Detection:**  Before each move, the code checks if the current player has any immediate winning moves available. This information is recorded in the match data.
*   **Move Recording:**  Each move is recorded, including the chosen column, the player, and whether immediate win positions were available.
*   **JSON Serialization:** The generated match data is serialized to a JSON file (`matches_lite.json`) for easy storage and later analysis.
*   **Match Replay & Visualization:**  A function is provided to reconstruct the board state after each move of a specific match and print it to the console using Unicode circles.
*   **Clear Board Representation:** Uses Unicode circles (🟡, 🔴, ⚪) to visually represent the board state.
* **Modularity** Functions have been created to improve code.

## Dependencies

This project relies on the following crates:

*   [`rand`](https://crates.io/crates/rand): For random number generation.
*   [`rayon`](https://crates.io/crates/rayon): For parallel processing.
*   [`serde`](https://crates.io/crates/serde): For serialization and deserialization (with `derive` feature).
*   [`serde_json`](https://crates.io/crates/serde_json):  For JSON serialization.

These dependencies are declared in the `Cargo.toml` file and will be automatically downloaded and built by Cargo.

## Building and Running

1.  **Prerequisites:** Make sure you have Rust and Cargo installed.  You can install them from [the official Rust website](https://www.rust-lang.org/tools/install).

2.  **Clone the Repository:**

    ```bash
    git clone https://github.com/TheGhoul21/connect-4-gen.git
    cd connect-4-gen
    ```
3.  **Build the Project:**

    ```bash
    cargo build --release
    ```
    This command compiles the project in release mode, creating an optimized executable.

4.  **Run the Project:**

    ```bash
    cargo run --release
    ```
    This will:
    *   Generate 1000 Connect4 matches (this number is configurable in `main.rs`).
    *   Save the match data to `matches_lite.json` in the project's root directory.
    *   Replay and print the board states for match #2 (also configurable).

## Code Structure

*   **`Player` Enum:** Represents the two players (Yellow and Red).
*   **`Board` Struct:** Represents the Connect4 board. Includes methods for:
    *   `new()`: Creates a new, empty board.
    *   `can_play(col)`: Checks if a move in the given column is valid.
    *   `play(col, player)`: Attempts to place a piece in the given column.  Returns the row/col position if successful, or `None` if the move is invalid.
    *   `is_winning_move(row, col, player)`: Checks if the move at (row, col) resulted in a win for the given player.
    * `immediate_wins(player)` Check if the current player has any immediate winning moves available.
*   **`MoveRecord` Struct:** Stores information about a single move:
    *   `usr_move`: The column chosen by the player.
    *   `has_immediate_win`: Whether there was at least one winning move in the previous turn
    * `immediate_win_positions`: Positions of immediate win
    *   `player`: The player who made the move.
*   **`random_connect4_match(rng)`:**  Simulates a single Connect4 match, returning a vector of `MoveRecord`s.
*   **`print_board(board)`:** Prints the current state of the board to the console.
*   **`print_match_moves(moves)`:** Replays and prints the board after each move in a given match.
*   **`main()`:**
    *   Generates the specified number of matches in parallel.
    *   Serializes the results to JSON.
    *   Replays and prints a chosen match.

## Configuration

You can easily modify a few parameters within the `main()` function:

*   `num_matches`: Change this variable to control the number of simulated matches.
*   `chosen_index`:  Change this variable to select which match will be replayed and printed.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.  (You'll need to create a `LICENSE` file in your repository and put the MIT license text in it.)

## Contributing

Contributions are welcome!  If you find a bug or have a feature suggestion, please open an issue or submit a pull request.  Before submitting a pull request, please ensure your code is formatted correctly (using `cargo fmt`) and that all tests pass (using `cargo test`).
## Future Improvements
*  Add unit tests.
*  Implement a more sophisticated AI (beyond random moves).
* Create more advanced analysis of matches.
*  Add a command-line interface for easier configuration.