use crate::pieces::Piece;

pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub promotion: Option<Piece>,
}

impl Move {
    pub fn from_algebraic_notation(input: &str) -> Option<Self> {
        let from_file = input.chars().nth(0)?;
        let from_rank = input.chars().nth(1)?;
        let to_file = input.chars().nth(2)?;
        let to_rank: char = input.chars().nth(3)?;

        let from = (
            8 - from_rank.to_digit(10)? as usize,
            (from_file as u8 - b'a') as usize,
        );
        let to = (
            8 - to_rank.to_digit(10)? as usize,
            (to_file as u8 - b'a') as usize,
        );

        Some(Move {
            from,
            to,
            promotion: None,
        })
    }
}
