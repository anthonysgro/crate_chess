#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct ChessPiece {
    pub piece_type: Piece,
    pub color: Color,
}

impl ChessPiece {
    pub fn new(piece_type: Piece, color: Color) -> Self {
        ChessPiece { piece_type, color }
    }

    // You can later add movement rules or other logic here if needed
}