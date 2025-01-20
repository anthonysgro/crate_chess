use crate::{pieces::ChessPiece, tile::Tile};


#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: Tile,
    pub to: Tile,
    pub piece: ChessPiece,
    pub promotion: Option<ChessPiece>,
}

impl Move {
    pub fn new(from: Tile, to: Tile, piece: ChessPiece, promotion: Option<ChessPiece>) -> Self {
        Self { from, to, piece, promotion }
    }
}
