mod pieces;
mod board;
mod r#move;
mod chess;
mod fen;
mod castling_rights;
mod tile;

fn main() {
    let fen = "1k6/8/8/8/1b6/8/3Q1N2/1K6 w - - 0 1";

    // Create a new Chess game with a board from the provided FEN string
    let game = chess::Chess::from_fen(fen);
    // let game = chess::Chess::default();

    // Print the board to verify the setup
    game.board.pretty_print();

    println!("Turn: {:?}", game.get_turn());
    println!("{:?}", game.castling_rights);
    println!("En Passant Target: {:?}", game.en_passant_target);
    println!("Halfmove Clock: {}", game.halfmove_clock);
    println!("Fullmove Number: {}", game.fullmove_number);

    println!("Legal Moves: [");
    for legal_move in game.get_legal_moves().iter() {
        println!("  {{\n    from: {:?}\n    to: {:?}\n    piece: {:?}\n    promotion: {:?}\n  }},\n", legal_move.from.name.get_notation_name(), legal_move.to.name.get_notation_name(), legal_move.piece, legal_move.promotion);
    }
    println!("]")

}
