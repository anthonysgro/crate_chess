use crate::pieces::{ChessPiece, Piece, Color};
use crate::tile::Tile;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub position: [Tile; 64], // Fixed-size array for the board
}

impl Board {
    pub fn index(x: usize, y: usize) -> usize {
        y * 8 + x    
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.position[Self::index(x, y)]
    }
    
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.position[Self::index(x, y)]
    }

    pub fn get_tile_with_name(&self, name: &str) -> &Tile {
        if name.len() != 2 {
            panic!("Invalid tile name");
        }
    
        let file = name.chars().next().unwrap();  // 'a' to 'h'
        let rank = name.chars().nth(1).unwrap(); // '1' to '8'
        let x = file as usize - 'a' as usize;
        let y = (rank.to_digit(10).unwrap() - 1) as usize;
        println!("x: {}, y: {}", x, y);
    
        if x >= 8 || y >= 8 {
            panic!("Invalid coordinates for tile: {}", name);
        }
    
        &self.position[Self::index(x, y)]
    }

    pub fn get_tile_with_name_mut(&mut self, name: &str) -> &mut Tile {
        if name.len() != 2 {
            panic!("Invalid tile name");
        }
    
        let file = name.chars().next().unwrap();  // 'a' to 'h'
        let rank = name.chars().nth(1).unwrap(); // '1' to '8'
        let x = file as usize - 'a' as usize;
        let y = 8 - rank.to_digit(10).unwrap() as usize;
    
        if x >= 8 || y >= 8 {
            panic!("Invalid coordinates for tile: {}", name);
        }
    
        &mut self.position[Self::index(x, y)]
    }

    pub fn is_on_board(&self, x: usize, y: usize) -> bool {
        x < 8 && y < 8
    }

    // Initialize the board with empty tiles and proper names
    pub fn init() -> Self {
        let mut position: [Tile; 64] = [Tile::new(None, "a1"); 64];
        
        for y in 0..8 {
            for x in 0..8 {
                let file = (b'a' + x as u8) as char; // 'a' to 'h'
                let rank = y + 1; // '1' to '8'
                let name = format!("{}{}", file, rank); // e.g., "a8", "b8", ..., "h1"
                position[Self::index(x, y)] = Tile::new(None, &name);
            }
        }

        Self { position }
    }


    // Create a new board with the default starting position
    pub fn default() -> Self {
        let mut board: Board = Self::init();
        
        // Set up the default starting pieces
        for i in 0..8 {
            board.get_tile_mut(i, 1).set_piece(ChessPiece::new(Piece::Pawn, Color::White));
            board.get_tile_mut(i, 6).set_piece(ChessPiece::new(Piece::Pawn, Color::Black));
        }

        board.get_tile_mut(0, 0).set_piece(ChessPiece::new(Piece::Rook, Color::White));
        board.get_tile_mut(7, 0).set_piece(ChessPiece::new(Piece::Rook, Color::White));
        board.get_tile_mut(0, 7).set_piece(ChessPiece::new(Piece::Rook, Color::Black));
        board.get_tile_mut(7, 7).set_piece(ChessPiece::new(Piece::Rook, Color::Black));

        board.get_tile_mut(1, 0).set_piece(ChessPiece::new(Piece::Knight, Color::White));
        board.get_tile_mut(6, 0).set_piece(ChessPiece::new(Piece::Knight, Color::White));
        board.get_tile_mut(1, 7).set_piece(ChessPiece::new(Piece::Knight, Color::Black));
        board.get_tile_mut(6, 7).set_piece(ChessPiece::new(Piece::Knight, Color::Black));

        board.get_tile_mut(2, 0).set_piece(ChessPiece::new(Piece::Bishop, Color::White));
        board.get_tile_mut(5, 0).set_piece(ChessPiece::new(Piece::Bishop, Color::White));
        board.get_tile_mut(2, 7).set_piece(ChessPiece::new(Piece::Bishop, Color::Black));
        board.get_tile_mut(5, 7).set_piece(ChessPiece::new(Piece::Bishop, Color::Black));

        board.get_tile_mut(3, 0).set_piece(ChessPiece::new(Piece::Queen, Color::White));
        board.get_tile_mut(3, 7).set_piece(ChessPiece::new(Piece::Queen, Color::Black));

        board.get_tile_mut(4, 0).set_piece(ChessPiece::new(Piece::King, Color::White));
        board.get_tile_mut(4, 7).set_piece(ChessPiece::new(Piece::King, Color::Black));
        
        Self { position: board.position }
    }

    pub fn from_fen(fen_board: &str) -> Self {
        let mut board: Board = Self::init();
    
        let rows: Vec<&str> = fen_board.split('/').collect();
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
                    let x = col;
                    let y = i; // Reverse to match board's visual layout

                    // Use index(x, y) to access the position in the fixed array
                    board.get_tile_mut(x, y).set_piece(ChessPiece::new(piece, color));
    
                    col += 1;
                }
            }
        }

        board
    }
    


    pub fn pretty_print(&self) {
        println!("  +------------------------+");
        for y in (0..8).rev() { // Print from row 8 down to row 1
            print!("{} |", y + 1); // Row numbers
            for x in 0..8 {
                let tile = &self.position[Self::index(x, y)];
                match &tile.piece {
                    Some(piece) => print!(" {:<2}", self.piece_to_unicode(piece)),
                    None => print!(" . "), // Empty tile representation
                    // None => print!(" {:<5} ", tile),  // Uses the Display trait for tile name
                }
            }
            println!("|");
        }
        println!("  +------------------------+");
        println!("    a  b  c  d  e  f  g  h"); // Column labels
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
