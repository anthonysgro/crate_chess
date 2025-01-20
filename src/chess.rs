use crate::board::Board;
use crate::pieces::{Color, ChessPiece};

pub struct Chess {
    pub board: Board,
    pub turn: Color, // Track whose turn it is
}

impl Chess {
    pub fn new() -> Self {
        // Create a new Chess game, starting with a default board and White to move
        Chess {
            board: Board::new(),
            turn: Color::White,
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

    // Create a new Chess game and set up the board and turn from FEN
    pub fn from_fen(fen: &str) -> Self {

        // Validate FEN
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            panic!("Invalid FEN format");
        }
    
        // Initialize the Chess game properties
        let mut game = Chess {
            board: Board::new(), // Start with a default board
            turn: Color::White,  // Default turn (White)
        };
    
        // Set up the board from the FEN string
        game.board.from_fen(parts[0]);
    
        // Determine whose turn it is based on FEN
        game.turn = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid turn color in FEN"),
        };
    
        game
    }

    // Additional methods for game state management (check, checkmate, win condition, etc.)
}
