mod pieces;
mod board;
mod r#move;
mod chess;
mod fen;
mod castling_rights;
mod tile;

fn main() {
    let fen = "r1bqk2r/ppp2ppp/2nb4/1B1np3/4P3/2N2N2/PP1P1PPP/R1BQK2R b KQkq - 0 7";

    // Create a new Chess game with a board from the provided FEN string
    let game = chess::Chess::from_fen(fen);
    // let game = chess::Chess::default();

    // Print the board to verify the setup
    game.board.pretty_print();

    println!("Turn: {:?}", game.get_turn());
    println!("{:?}", game.castling_rights);
}
