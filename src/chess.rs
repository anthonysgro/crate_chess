use crate::board::Board;
use crate::pieces::Color;
use crate::fen::Fen;
use crate::castling_rights::CastlingRights;
use crate::tile::Tile;

const WHITE: &str = "w";
const BLACK: &str = "b";

pub struct Chess {
    pub board: Board,
    pub turn: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Tile>,
}

impl Chess {
    // Create a new Chess game, starting with an empty board and White to move
    pub fn empty() -> Self {
        Chess {
            board: Board::init(),
            turn: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_target: None,
        }
    }

    // Create a new Chess game, starting with a default board and White to move
    pub fn default() -> Self {
        Chess {
            board: Board::default(),
            turn: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_target: None,
        }
    }

    // Create a new Chess game from a FEN string
    pub fn from_fen(fen: &str) -> Self {
        let valid_fen: bool = Fen::validate_fen(fen);
        let parts: Vec<&str> = fen.split_whitespace().collect();
        
        if valid_fen == true {
            let board = Board::from_fen(parts[0]);
            Chess {
                board: board,
                turn: match parts[1] {
                    WHITE => Color::White,
                    BLACK => Color::Black,
                    _ => panic!("Invalid turn color in FEN"),
                },
                castling_rights: CastlingRights::from_rights(parts[2]),
                en_passant_target: match parts[3] {
                    "-" => None,
                    _ => Some(*board.get_tile_with_name(parts[3])),
                },
            }
        } else {
            panic!("Invalid FEN format");
        }
    }


    pub fn get_turn(&self) -> Color {
        self.turn
    }


    pub fn switch_turn(&mut self) {
        // Switch to the other player's turn
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    // Additional methods for game state management (check, checkmate, win condition, etc.)
}
