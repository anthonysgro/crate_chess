// main.rs
mod pieces;
mod board;

fn main() {
    let board = board::Board::new();
    board.pretty_print(); // Show the initial setup
}