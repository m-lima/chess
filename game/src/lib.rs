#![deny(warnings, clippy::pedantic, clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
// Allowed because it is wasm
#![allow(clippy::must_use_candidate)]

enum Token {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

enum Cell {
    Black,
    White,
}

#[wasm_bindgen]
pub fn get_candidates(board: &Board) {}
