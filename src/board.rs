// board.rs
use crate::pieces::{ChessPiece, Piece, Color};

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
