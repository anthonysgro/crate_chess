use crate::pieces::ChessPiece;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct TileName {
    pub name: u8,
}

impl TileName {
    pub fn new(name: &str) -> Self {
        let mut name = name.chars();
        let file = name.next().unwrap() as u8 - 'a' as u8;
        let rank = name.next().unwrap() as u8 - '1' as u8;
        TileName {
            name: file + rank * 8,
        }
    }

    pub fn get_name(&self) -> String {
        let file = (self.name % 8 + 'a' as u8) as u8 as char;
        let rank = (self.name / 8 + '1' as u8) as u8 as char;
        format!("{}{}", file, rank)
    }
}

impl fmt::Display for TileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = (self.name % 8 + 'a' as u8) as u8 as char;
        let rank = (self.name / 8 + '1' as u8) as u8 as char;
        write!(f, "{}{}", file, rank)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub piece: Option<ChessPiece>,
    pub en_passant_available: bool,
    pub name: TileName,
}

impl Tile {
    pub fn new(piece: Option<ChessPiece>, name: &str) -> Self {
        Tile {
            piece,
            en_passant_available: false,
            name: TileName::new(name),
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

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.name, 
        )
    }
}