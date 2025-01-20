
#[derive(Debug, Clone)]
pub struct Fen {
    pub board: String,        // The board setup (like "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    pub turn: char,           // Whose turn it is ('w' for white, 'b' for black)
    pub castling: String,     // Castling rights ('KQkq' or empty string if no rights)
    pub en_passant: String,   // En passant target square (or '-' if none)
    pub halfmove_clock: u8,   // Halfmove clock (for fifty-move rule)
    pub fullmove_number: u8,  // Fullmove number (starts at 1)
}

impl Fen {
    pub fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            panic!("Invalid FEN string, expected 6 parts");
        }

        let board = parts[0].to_string();
        let turn = parts[1].chars().next().unwrap();
        let castling = parts[2].to_string();
        let en_passant = parts[3].to_string();
        let halfmove_clock = parts[4].parse().unwrap_or(0);
        let fullmove_number = parts[5].parse().unwrap_or(1);

        Fen {
            board,
            turn,
            castling,
            en_passant,
            halfmove_clock,
            fullmove_number,
        }
    }

    pub fn to_fen(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.board,
            self.turn,
            self.castling,
            self.en_passant,
            self.halfmove_clock,
            self.fullmove_number
        )
    }
}