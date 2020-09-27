#![deny(warnings, clippy::pedantic, clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]

pub mod bind;
pub mod model;

pub use model::Board;
pub use model::Color;
pub use model::Token;

pub fn possible_moves(index: u8, board: &Board) -> Vec<u8> {
    if let Some(ref cell) = board.cells()[usize::from(index)] {
        match cell.token() {
            Token::Pawn => match cell.color() {
                Color::Black => {
                    if index / 8 == 1 {
                        vec![index + 8, index + 16]
                    } else {
                        vec![index + 8]
                    }
                }
                Color::White => {
                    if index / 8 == 6 {
                        vec![index - 8, index - 16]
                    } else {
                        vec![index - 8]
                    }
                }
            },
            _ => vec![],
        }
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pawn_initial_move() {
        let board: super::Board::new();

        assert_eq!(super::possible_moves(8, &board), vec![16, 24]);
        assert_eq!(super::possible_moves(55, &board), vec![47, 39]);
    }
}
