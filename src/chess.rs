use crate::board::Board;
use crate::pieces::Color;
use crate::fen::Fen;
use crate::castling_rights::CastlingRights;
use crate::tile::Tile;

pub struct Chess {
    pub board: Board,
    pub turn: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Tile>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
}

impl Chess {
    // Create a new Chess game, starting with an empty board and White to move
    pub fn empty() -> Self {
        Chess {
            board: Board::init(),
            turn: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_target: None,
            fullmove_number: 1,
            halfmove_clock: 0,
        }
    }

    // Create a new Chess game, starting with a default board and White to move
    pub fn default() -> Self {
        Chess {
            board: Board::default(),
            turn: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_target: None,
            fullmove_number: 1,
            halfmove_clock: 0,
        }
    }

    // Create a new Chess game from a FEN string
    pub fn from_fen(fen: &str) -> Self {
        let fen: Fen = Fen::from_fen(fen);
        
        let board = Board::from_fen(&fen.board, &fen.en_passant);
        Chess {
            board,
            turn: match &fen.turn {
                'w' => Color::White,
                'b' => Color::Black,
                _ => panic!("Invalid turn color in FEN"),
            },
            castling_rights: CastlingRights::from_rights(&fen.castling),
            en_passant_target: match fen.en_passant.as_str() {
                "-" => None,
                _ => Some(*board.get_tile_with_name(&fen.en_passant)),
            },
            halfmove_clock: fen.halfmove_clock,
            fullmove_number: fen.fullmove_number,
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
