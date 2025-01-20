// main.rs
mod pieces;
mod board;
mod r#move;

fn main() {
    // let board = board::Board::new();

    // let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen = "r1bqk2r/ppp2ppp/2nb4/1B1np3/8/2N1PN2/PP1P1PPP/R1BQK2R w KQkq - 4 7";
    let board = board::Board::from_fen(fen);

    board.pretty_print(); // Show the initial setup
}