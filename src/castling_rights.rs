use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingRights {
    pub fn default() -> Self {
        CastlingRights {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }

    pub fn from_rights(rights: &str) -> Self {
        CastlingRights {
            white_king_side: rights.contains('K'),
            white_queen_side: rights.contains('Q'),
            black_king_side: rights.contains('k'),
            black_queen_side: rights.contains('q'),
        }
    }

    pub fn set_castling_rights(&mut self, rights: &str) {
        self.white_king_side = rights.contains('K');
        self.white_queen_side = rights.contains('Q');
        self.black_king_side = rights.contains('k');
        self.black_queen_side = rights.contains('q');
    }
}