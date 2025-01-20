use crate::board::Board;
use crate::pieces::{Color, ChessPiece};
use crate::fen::Fen;

const WHITE: &str = "w";
const BLACK: &str = "b";

pub struct Chess {
    pub board: Board,
    pub turn: Color, // Track whose turn it is
}

impl Chess {
    // Create a new Chess game, starting with an empty board and White to move
    pub fn empty() -> Self {
        Chess {
            board: Board::new(),
            turn: Color::White,
        }
    }

    // Create a new Chess game, starting with a default board and White to move
    pub fn default() -> Self {
        Chess {
            board: Board::default(),
            turn: Color::White,
        }
    }

    // Create a new Chess game from a FEN string
    pub fn from_fen(fen: &str) -> Self {
        let valid_fen: bool = Fen::validate_fen(fen);
        let parts: Vec<&str> = fen.split_whitespace().collect();
        
        if valid_fen == true {
            Chess {
                board: Board::from_fen(parts[0]),
                turn: match parts[1] {
                    WHITE => Color::White,
                    BLACK => Color::Black,
                    _ => panic!("Invalid turn color in FEN"),
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

    pub fn make_move(&mut self, chess_move: &str) -> Result<(), String> {
        // Parse the move (you can add move validation and logic here)
        // Example: Convert algebraic notation to the internal move representation

        // After a successful move, switch turns
        self.switch_turn();

        Ok(())
    }

    // Additional methods for game state management (check, checkmate, win condition, etc.)
}
