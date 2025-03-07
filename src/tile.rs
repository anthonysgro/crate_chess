use crate::pieces::ChessPiece;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct TileName {
    pub idx: u8,
}

impl TileName {
    pub fn new(name: &str) -> Self {
        let mut name = name.chars();
        let file = name.next().unwrap() as u8 - 'a' as u8;
        let rank = name.next().unwrap() as u8 - '1' as u8;
        TileName {
            idx: file + rank * 8,
        }
    }

    pub fn get_name(&self) -> u8 {
        self.idx
    }

    pub fn get_notation_name(&self) -> String {
        let file = (self.idx % 8 + 'a' as u8) as u8 as char;
        let rank = (self.idx / 8 + '1' as u8) as u8 as char;
        format!("{}{}", file, rank)
    }
}

impl fmt::Display for TileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = (self.idx % 8 + 'a' as u8) as u8 as char;
        let rank = (self.idx / 8 + '1' as u8) as u8 as char;
        write!(f, "{}{}", file, rank)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub piece: Option<ChessPiece>,
    pub name: TileName,
}

impl Tile {
    pub fn new(piece: Option<ChessPiece>, name: &str) -> Self {
        Tile {
            piece,
            name: TileName::new(name),
        }
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

    pub fn get_coords(&self) -> (usize, usize) {
        let x = (self.name.idx % 8) as usize;
        let y = (self.name.idx / 8) as usize;
        (x, y)
    }

}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tile {{ piece: {:?}, name: {} }}", 
          self.piece, 
          self.name
        )
    }
}