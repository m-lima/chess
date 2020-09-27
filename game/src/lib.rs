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
    None,
    WhiteCheckmates,
    WhiteResigns,
    BlackCheckmates,
    BlackResigns,
    Stalemate,
    DrawAccepted,
    DrawDeclared,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
}

#[wasm_bindgen]
pub struct Board {
    game: chess::Game,
    moves: Vec<chess::ChessMove>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let game = chess::Game::new();
        Self {
            moves: chess::MoveGen::new_legal(&game.current_position()).collect(),
            game,
        }
    }

    #[wasm_bindgen]
    pub fn to_fen(&self) -> js_sys::JsString {
        self.game.current_position().to_string().into()
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
        match self.game.side_to_move() {
            chess::Color::Black => Color::Black,
            chess::Color::White => Color::White,
        }
    }

    #[wasm_bindgen]
    pub fn result(&self) -> State {
        match self.game.result() {
            None => State::None,
            Some(chess::GameResult::Stalemate) => State::Stalemate,
            Some(chess::GameResult::DrawDeclared) => State::DrawDeclared,
            Some(chess::GameResult::DrawAccepted) => State::DrawAccepted,
            Some(chess::GameResult::BlackResigns) => State::BlackResigns,
            Some(chess::GameResult::BlackCheckmates) => State::BlackCheckmates,
            Some(chess::GameResult::WhiteResigns) => State::WhiteResigns,
            Some(chess::GameResult::WhiteCheckmates) => State::WhiteCheckmates,
        }
    }
}
