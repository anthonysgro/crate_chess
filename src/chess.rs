use crate::board::Board;
use crate::r#move::Move;
use crate::pieces::{ChessPiece, Color, Piece};
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

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();

        // Iterate over all tiles and find pieces of the correct color
        for tile in self.board.position.iter() {
            if let Some(piece) = &tile.piece {
                if piece.color == self.turn {
                    // Get possible moves for the piece on this tile
                    let possible_moves = piece.get_possible_moves(*tile, &self.board, self.castling_rights);
                    
                    // let possible_moves = piece.get_possible_moves(self, *piece, tile);
                    legal_moves.extend(possible_moves);
                }
            }
        }
        legal_moves
    }

    fn is_valid_square(&self, x: usize, y: usize) -> bool {
        // Ensure that the square is within bounds
        x < 8 && y < 8
    }

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index % 8, index / 8) // Convert index to x, y coordinates
    }

    // Additional methods for game state management (check, checkmate, win condition, etc.)
}
