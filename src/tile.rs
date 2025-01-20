use crate::pieces::ChessPiece;

#[derive(Debug, Clone)]
pub struct Tile {
    pub piece: Option<ChessPiece>,
    pub en_passant_available: bool,
    pub name: String,
}

impl Tile {
    pub fn new(piece: Option<ChessPiece>, name: String) -> Self {
        Tile {
            piece,
            en_passant_available: false,
            name,
        }
    }

    pub fn set_en_passant_available(&mut self, is_en_passant_target_suare: bool) {
        self.en_passant_available = is_en_passant_target_suare;
    }

    pub fn is_en_passant_available(&self) -> bool {
        self.en_passant_available
    }

    pub fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }

    pub fn set_piece(&mut self, piece: ChessPiece) {
        self.piece = Some(piece);
    }

    pub fn clear(&mut self) {
        self.piece = None;
    }

}