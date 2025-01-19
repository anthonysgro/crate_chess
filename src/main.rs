// main.rs
mod pieces;
mod board;

fn main() {
    let board = board::Board::new();
    board.display(); // Show the initial setup
}