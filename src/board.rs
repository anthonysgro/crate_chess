use crate::pieces::{ChessPiece, Piece, Color};
use crate::r#move::Move;

pub struct Board {
    pub squares: Vec<Vec<Option<ChessPiece>>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = vec![vec![None; 8]; 8]; // 8x8 chess board

        // Set up initial pieces
        for i in 0..8 {
            board[1][i] = Some(ChessPiece::new(Piece::Pawn, Color::White)); // White pawns
            board[6][i] = Some(ChessPiece::new(Piece::Pawn, Color::Black)); // Black pawns
        }

        // Set up rooks
        board[0][0] = Some(ChessPiece::new(Piece::Rook, Color::White));
        board[0][7] = Some(ChessPiece::new(Piece::Rook, Color::White));
        board[7][0] = Some(ChessPiece::new(Piece::Rook, Color::Black));
        board[7][7] = Some(ChessPiece::new(Piece::Rook, Color::Black));

        // Set up knights
        board[0][1] = Some(ChessPiece::new(Piece::Knight, Color::White));
        board[0][6] = Some(ChessPiece::new(Piece::Knight, Color::White));
        board[7][1] = Some(ChessPiece::new(Piece::Knight, Color::Black));
        board[7][6] = Some(ChessPiece::new(Piece::Knight, Color::Black));

        // Set up bishops
        board[0][2] = Some(ChessPiece::new(Piece::Bishop, Color::White));
        board[0][5] = Some(ChessPiece::new(Piece::Bishop, Color::White));
        board[7][2] = Some(ChessPiece::new(Piece::Bishop, Color::Black));
        board[7][5] = Some(ChessPiece::new(Piece::Bishop, Color::Black));

        // Set up queens
        board[0][3] = Some(ChessPiece::new(Piece::Queen, Color::White));
        board[7][3] = Some(ChessPiece::new(Piece::Queen, Color::Black));

        // Set up kings
        board[0][4] = Some(ChessPiece::new(Piece::King, Color::White));
        board[7][4] = Some(ChessPiece::new(Piece::King, Color::Black));

        Board { squares: board }
    }

    pub fn apply_move(&mut self, chess_move: &Move) -> Result<(), &'static str> {
        let (from_x, from_y) = chess_move.from;
        let (to_x, to_y) = chess_move.to;

        if from_x >= 8 || from_y >= 8 || to_x >= 8 || to_y >= 8 {
            return Err("Move is out of bounds");
        }

        let piece: Option<ChessPiece> = self.squares[from_x][from_y].take();

        match piece {
            Some(chess_piece) => {
                // Add detailed move validation here

                self.squares[to_x][to_y] = Some(chess_piece);
                Ok(())
            }
            None => Err("No piece at the starting position"),
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = vec![vec![None; 8]; 8]; // 8x8 chess board
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let piece_placement = parts[0];
        let rows: Vec<&str> = piece_placement.split('/').collect();

        for (row_idx, row) in rows.iter().enumerate() {
            let mut col_idx = 0;
            for ch in row.chars() {
                if ch.is_digit(10) {
                    col_idx += ch.to_digit(10).unwrap() as usize;
                } else {
                    let (piece, color) = match ch {
                        'r' => (Piece::Rook, Color::Black),
                        'n' => (Piece::Knight, Color::Black),
                        'b' => (Piece::Bishop, Color::Black),
                        'q' => (Piece::Queen, Color::Black),
                        'k' => (Piece::King, Color::Black),
                        'p' => (Piece::Pawn, Color::Black),
                        'R' => (Piece::Rook, Color::White),
                        'N' => (Piece::Knight, Color::White),
                        'B' => (Piece::Bishop, Color::White),
                        'Q' => (Piece::Queen, Color::White),
                        'K' => (Piece::King, Color::White),
                        'P' => (Piece::Pawn, Color::White),
                        _ => panic!("Invalid FEN character: {}", ch),
                    };
                    board[row_idx][col_idx] = Some(ChessPiece::new(piece, color));
                    col_idx += 1;
                }
            }
        }

        Board { squares: board }
    }

    pub fn pretty_print(&self) {
        println!(" +------------------------+");
        for (i, row) in self.squares.iter().enumerate() {
            print!("{}|", 8 - i); // Row numbers
            for square in row {
                match square {
                    Some(piece) => print!(" {:<2}", self.piece_to_unicode(piece)),
                    None => print!(" . "),
                }
            }
            println!("|");
        }
        println!(" +------------------------+");
        println!("   a  b  c  d  e  f  g  h");
    }

    fn piece_to_unicode(&self, piece: &ChessPiece) -> char {
        match piece.piece_type {
            Piece::King => match piece.color {
                Color::White => '♔',
                Color::Black => '♚',
            },
            Piece::Queen => match piece.color {
                Color::White => '♕',
                Color::Black => '♛',
            },
            Piece::Rook => match piece.color {
                Color::White => '♖',
                Color::Black => '♜',
            },
            Piece::Bishop => match piece.color {
                Color::White => '♗',
                Color::Black => '♝',
            },
            Piece::Knight => match piece.color {
                Color::White => '♘',
                Color::Black => '♞',
            },
            Piece::Pawn => match piece.color {
                Color::White => '♙',
                Color::Black => '♟',
            },
        }
    }
}
