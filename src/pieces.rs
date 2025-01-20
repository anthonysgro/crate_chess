use crate::board::Board;
use crate::r#move::Move;
use crate::tile::Tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct ChessPiece {
    pub piece_type: Piece,
    pub color: Color,
}

impl ChessPiece {
    pub fn new(piece_type: Piece, color: Color) -> Self {
        ChessPiece { piece_type, color }
    }

    pub fn get_possible_moves(&self, origin_tile: Tile, board: &Board) -> Vec<Move> {
        match self.piece_type {
            Piece::Pawn => self.get_pawn_moves(origin_tile, board),
            Piece::Knight => self.get_knight_moves(origin_tile, board),
            Piece::Rook => self.get_rook_moves(origin_tile, board),
            Piece::Bishop => self.get_bishop_moves(origin_tile, board),
            Piece::Queen => self.get_queen_moves(origin_tile, board),
            Piece::King => self.get_king_moves(origin_tile, board),
        }
    }

    fn get_pawn_moves(&self, origin_tile: Tile, board: &Board) -> Vec<Move> {
        let mut moves= Vec::new();
        let (x, y) = origin_tile.get_coords();

        let direction = if self.color == Color::White { 1 } else { -1 };

        // Default forward move 1
        let one_row_forward = (y as isize + direction) as usize;

        if board.is_on_board(x, one_row_forward) {
            let destination_tile = *board.get_tile(x, one_row_forward);
            if !destination_tile.is_occupied() {
                if (self.color == Color::White && y == 6) || (self.color == Color::Black && y == 1) {
                    // Promotion
                    let promotion = [
                        ChessPiece::new(Piece::Queen, self.color), 
                        ChessPiece::new(Piece::Rook, self.color), 
                        ChessPiece::new(Piece::Bishop, self.color), 
                        ChessPiece::new(Piece::Knight, self.color)
                    ];
                    for piece in promotion {
                        let pawn_move = Move::new(origin_tile, destination_tile, *self, Some(piece));
                        moves.push(pawn_move);
                    }
                } else {
                    // Non-promotion
                    let pawn_move: Move = Move::new(origin_tile, destination_tile, *self, None);
                    moves.push(pawn_move);
                }
            }    
        }

        // Pawn can move 2 spaces forward from starting position
        if self.color == Color::White && y == 1 || self.color == Color::Black && y == 6 {
            let two_rows_forward = (y as isize + direction * 2) as usize;
            let destination_tile = board.get_tile(x, two_rows_forward);
            let one_row_forward_tile = board.get_tile(x, one_row_forward);
            if !destination_tile.is_occupied() && !one_row_forward_tile.is_occupied() {
                let pawn_move = Move::new(origin_tile, *destination_tile, *self, None);
                moves.push(pawn_move);
            }
        } 

        // Diagonal capture left
        let new_x_left = (x as isize - 1) as usize;
        let new_y_left = (y as isize + direction) as usize;
        if board.is_on_board(new_x_left, new_y_left) {
            let destination_tile = board.get_tile(new_x_left, new_y_left);
            if let Some(piece) = destination_tile.piece {
                if piece.color != self.color {
                    if (self.color == Color::White && y == 6) || (self.color == Color::Black && y == 1) {
                        // Promotion
                        let promotion = [
                            ChessPiece::new(Piece::Queen, self.color), 
                            ChessPiece::new(Piece::Rook, self.color), 
                            ChessPiece::new(Piece::Bishop, self.color), 
                            ChessPiece::new(Piece::Knight, self.color)
                        ];                        
                    
                        for piece in promotion {
                            let pawn_move: Move = Move::new(origin_tile, *destination_tile, *self, Some(piece));
                            moves.push(pawn_move);
                        }
                    } else {
                        // Non-Promotion
                        let pawn_move: Move = Move::new(origin_tile, *destination_tile, *self, None);
                        moves.push(pawn_move);
                    }
                }
            }
        }

        // Diagonal capture right
        let new_x_right = (x as isize + 1) as usize;
        let new_y_right = (y as isize + direction) as usize;
        if board.is_on_board(new_x_right, new_y_right) {
            let destination_tile = board.get_tile(new_x_right, new_y_right);
            if let Some(piece) = destination_tile.piece {
                if piece.color != self.color {
                    if (self.color == Color::White && y == 6) || (self.color == Color::Black && y == 1) {
                        // Promotion
                        let promotion = [
                            ChessPiece::new(Piece::Queen, self.color), 
                            ChessPiece::new(Piece::Rook, self.color), 
                            ChessPiece::new(Piece::Bishop, self.color), 
                            ChessPiece::new(Piece::Knight, self.color)
                        ];                        

                        for piece in promotion {
                            let pawn_move: Move = Move::new(origin_tile, *destination_tile, *self, Some(piece));
                            moves.push(pawn_move);
                        }
                    } else {
                        // Non-Promotion
                        let pawn_move: Move = Move::new(origin_tile, *destination_tile, *self, None);
                        moves.push(pawn_move);
                    }
                }
            }
        }
        
        moves
    }

    fn get_knight_moves(&self, origin_tile: Tile, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move>= Vec::new();
        let (x, y) = origin_tile.get_coords();

        // Possible L-shaped moves (8 possible moves)
        let knight_moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2)
        ];

        for &(dx, dy) in &knight_moves {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            // Check if the move is on the board
            if board.is_on_board(new_x, new_y) {
                let destination_tile = board.get_tile(new_x, new_y);

                // If the destination tile is empty or contains an opponent's piece
                if !destination_tile.is_occupied() || destination_tile.piece.unwrap().color != self.color {
                    let knight_move = Move::new(origin_tile, *destination_tile, *self, None);
                    moves.push(knight_move);
                }
            }
        }
        moves
    }

    fn get_rook_moves(&self, origin_tile: Tile, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move>= Vec::new();
        let (x, y) = origin_tile.get_coords();

        // Horizontal and Vertical Directions
        let directions: [(isize, isize); 4] = [
            (1, 0),   // Right
            (-1, 0),  // Left
            (0, 1),   // Down
            (0, -1),  // Up
        ];

        for (dx, dy) in directions.iter() {
            let mut new_x = x as isize;
            let mut new_y = y as isize;

            // Move in the current direction until hitting the edge or another piece
            loop {
                new_x += dx;
                new_y += dy;

                if !board.is_on_board(new_x as usize, new_y as usize) {
                    break;
                }

                let destination_tile = board.get_tile(new_x as usize, new_y as usize);

                if destination_tile.is_occupied() {
                    if destination_tile.piece.unwrap().color != self.color {
                        let rook_move = Move::new(origin_tile, *destination_tile, *self, None);
                        moves.push(rook_move);
                    }
                    break;
                } else {
                    let rook_move = Move::new(origin_tile, *destination_tile, *self, None);
                    moves.push(rook_move);
                }
            }
        }

        moves
    }

    fn get_bishop_moves(&self, origin_tile: Tile, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move>= Vec::new();
        let (x, y) = origin_tile.get_coords();

        // Diagonal Directions
        let directions: [(isize, isize); 4] = [
            (1, 1),    // Down-Right
            (-1, 1),   // Down-Left
            (1, -1),   // Up-Right
            (-1, -1),  // Up-Left
        ];

        for (dx, dy) in directions.iter() {
            let mut new_x = x as isize;
            let mut new_y = y as isize;

            // Move in the current direction until hitting the edge or another piece
            loop {
                new_x += dx;
                new_y += dy;

                if !board.is_on_board(new_x as usize, new_y as usize) {
                    break;
                }

                let destination_tile = board.get_tile(new_x as usize, new_y as usize);

                if destination_tile.is_occupied() {
                    if destination_tile.piece.unwrap().color != self.color {
                        let bishop_move = Move::new(origin_tile, *destination_tile, *self, None);
                        moves.push(bishop_move);
                    }
                    break;
                } else {
                    let bishop_move = Move::new(origin_tile, *destination_tile, *self, None);
                    moves.push(bishop_move);
                }
            }
        }

        moves
    }

    fn get_queen_moves(&self, origin_tile: Tile, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move>= Vec::new();
        moves.extend(self.get_rook_moves(origin_tile, board));
        moves.extend(self.get_bishop_moves(origin_tile, board));
        moves

        // Queen-specific move logic
    }

    fn get_king_moves(&self, origin_tile: Tile, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move>= Vec::new();
        moves

        // King-specific move logic
    }
}