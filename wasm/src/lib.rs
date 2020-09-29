use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8(a: u8);
}

fn translate_index(index: u8) -> u8 {
    let rank = (!index >> 3) & 7;
    let file = index & 7;
    (rank << 3) | file
}

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum State {
    Ongoing,
    Stalemate,
    Checkmate,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Piece {
    None,
    BlackPawn,
    BlackKnight,
    BlackRook,
    BlackBishop,
    BlackQueen,
    BlackKing,
    WhitePawn,
    WhiteKnight,
    WhiteRook,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
}

#[wasm_bindgen]
pub fn color_for(piece: Piece) -> Option<Color> {
    match piece {
        Piece::None => None,
        Piece::BlackPawn
        | Piece::BlackKnight
        | Piece::BlackRook
        | Piece::BlackBishop
        | Piece::BlackQueen
        | Piece::BlackKing => Some(Color::Black),
        Piece::WhitePawn
        | Piece::WhiteKnight
        | Piece::WhiteRook
        | Piece::WhiteBishop
        | Piece::WhiteQueen
        | Piece::WhiteKing => Some(Color::White),
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
}

#[wasm_bindgen]
pub struct Board {
    board: chess::Board,
    moves: Vec<chess::ChessMove>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: &str) -> Self {
        use std::str::FromStr;
        let board = chess::Board::from_str(fen).expect("valid fen");
        Self {
            moves: chess::MoveGen::new_legal(&board).collect(),
            board,
        }
    }

    #[wasm_bindgen]
    pub fn to_fen(&self) -> js_sys::JsString {
        self.board.to_string().into()
    }

    #[wasm_bindgen]
    pub fn enumerate(&self) -> js_sys::Array {
        self.board
            .to_string()
            .chars()
            .take_while(|c| c != &' ')
            .filter_map(|c| match c {
                'p' => Some(vec![Piece::BlackPawn]),
                'n' => Some(vec![Piece::BlackKnight]),
                'r' => Some(vec![Piece::BlackRook]),
                'b' => Some(vec![Piece::BlackBishop]),
                'q' => Some(vec![Piece::BlackQueen]),
                'k' => Some(vec![Piece::BlackKing]),
                'P' => Some(vec![Piece::WhitePawn]),
                'N' => Some(vec![Piece::WhiteKnight]),
                'R' => Some(vec![Piece::WhiteRook]),
                'B' => Some(vec![Piece::WhiteBishop]),
                'Q' => Some(vec![Piece::WhiteQueen]),
                'K' => Some(vec![Piece::WhiteKing]),
                empty @ '1'..='8' => Some(vec![Piece::None; empty as usize - '0' as usize]),
                _ => None,
            })
            .flat_map(|c| c)
            .map(|c| c as u8)
            .map(JsValue::from)
            .collect()
    }

    #[wasm_bindgen]
    pub fn legal_moves(&self, index: u8) -> js_sys::Array {
        if index < 64 {
            let square = unsafe { chess::Square::new(translate_index(index)) };
            self.moves
                .iter()
                .filter(|m| m.get_source() == square)
                .map(|m| JsValue::from(translate_index(m.get_dest().to_int())))
                .collect()
        } else {
            js_sys::Array::new_with_length(0)
        }
    }

    #[wasm_bindgen]
    pub fn side_to_move(&self) -> Color {
        match self.board.side_to_move() {
            chess::Color::Black => Color::Black,
            chess::Color::White => Color::White,
        }
    }

    #[wasm_bindgen]
    pub fn result(&self) -> State {
        match self.board.status() {
            chess::BoardStatus::Ongoing => State::Ongoing,
            chess::BoardStatus::Stalemate => State::Stalemate,
            chess::BoardStatus::Checkmate => State::Checkmate,
        }
    }
}
