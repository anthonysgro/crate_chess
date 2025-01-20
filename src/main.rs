mod pieces;
mod board;
mod r#move;
mod chess;
mod fen;
mod castling_rights;
mod tile;

fn main() {
    let fen = "rnbqkbnr/ppp1pppp/8/8/2PpP3/5P2/PP1P2PP/RNBQKBNR b KQkq c3 0 3";

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
    println!("Legal moves: {:?}", game.get_legal_moves());

    for tile in game.board.position.iter() {
        if let Some(piece) = tile.piece {
            println!("Piece: {:?} at {} at {:?}", piece, tile.name, tile.get_coords());
        }
    }

}
