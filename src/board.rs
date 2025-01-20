use crate::pieces::{ChessPiece, Piece, Color};
use crate::tile::Tile;

pub struct Board {
    pub position: Vec<Vec<Tile>>,
}

impl Board {

    pub fn init() -> Self {
        let mut board: Vec<Vec<Tile>> = Vec::with_capacity(8);
    
        for row in 0..8 {
            let mut row_tiles: Vec<Tile> = Vec::with_capacity(8);
            for col in 0..8 {
                // Generate name for the tile
                let name = format!("{}{}", (col as u8 + b'a') as char, row + 1); // Name like "a1", "b2", etc.
                let tile = Tile::new(None, name); // Empty tile
                row_tiles.push(tile);
            }
            board.push(row_tiles);
        }

        Board { position: board }
    }

    // Create a new board with the default starting position
    pub fn default() -> Self {
        let mut board: Vec<Vec<Tile>> = Board::init().position;

        // Set up the default starting pieces
        for i in 0..8 {
            board[1][i].set_piece(ChessPiece::new(Piece::Pawn, Color::White));
            board[6][i].set_piece(ChessPiece::new(Piece::Pawn, Color::Black));
        }

        // Set up other pieces (Rooks, Knights, Bishops, Queens, Kings)
        board[0][0].set_piece(ChessPiece::new(Piece::Rook, Color::White));
        board[0][7].set_piece(ChessPiece::new(Piece::Rook, Color::White));
        board[7][0].set_piece(ChessPiece::new(Piece::Rook, Color::Black));
        board[7][7].set_piece(ChessPiece::new(Piece::Rook, Color::Black));
        
        // Set up knights
        board[0][1].set_piece(ChessPiece::new(Piece::Knight, Color::White));
        board[0][6].set_piece(ChessPiece::new(Piece::Knight, Color::White));
        board[7][1].set_piece(ChessPiece::new(Piece::Knight, Color::Black));
        board[7][6].set_piece(ChessPiece::new(Piece::Knight, Color::Black));

        // Set up bishops
        board[0][2].set_piece(ChessPiece::new(Piece::Bishop, Color::White));
        board[0][5].set_piece(ChessPiece::new(Piece::Bishop, Color::White));
        board[7][2].set_piece(ChessPiece::new(Piece::Bishop, Color::Black));
        board[7][5].set_piece(ChessPiece::new(Piece::Bishop, Color::Black));
        
        // Set up queens
        board[0][3].set_piece(ChessPiece::new(Piece::Queen, Color::White));
        board[7][3].set_piece(ChessPiece::new(Piece::Queen, Color::Black));
        
        // Set up kings
        board[0][4].set_piece(ChessPiece::new(Piece::King, Color::White));
        board[7][4].set_piece(ChessPiece::new(Piece::King, Color::Black));
        
        Board { position: board }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board: Vec<Vec<Tile>> = Board::init().position;
    
        let rows: Vec<&str> = fen.split('/').collect();
        if rows.len() != 8 {
            panic!("Invalid FEN board setup");
        }
    
        // Reverse the rows to match board's visual layout (from bottom to top)
        for (i, row) in rows.iter().rev().enumerate() {
            let mut col = 0;
            for c in row.chars() {
                if c.is_digit(10) {
                    // If the character is a number, it represents empty squares
                    let empty_squares = c.to_digit(10).unwrap();
                    col += empty_squares as usize;
                } else {
                    // Otherwise, it's a piece
                    let (piece, color) = match c {
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
                        _ => panic!("Invalid piece character in FEN"),
                    };
                    // Insert the piece into the correct position in the board
                    board[i][col].set_piece(ChessPiece::new(piece, color));
                    col += 1;
                }
            }
        }
        Board { position: board }
    }
    
    pub fn pretty_print(&self) {
        println!(" +------------------------+");
    
        // Iterate through the rows in reverse order (from bottom to top)
        for row in self.position.iter().rev().enumerate() {
            let (i, row) = row;
            print!("{}|", 8 - i); // Row numbers (8 at the bottom, 1 at the top)
            for tile in row {
                match &tile.piece {
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
