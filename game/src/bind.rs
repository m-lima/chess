use wasm_bindgen::prelude::*;

use super::Board;

// #[wasm_bindgen]
pub fn possible_moves_js(index: u8, board: &Board) -> js_sys::Array {
    super::possible_moves(index, board)
        .into_iter()
        .map(JsValue::from)
        .collect()
}
