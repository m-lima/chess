use wasm_bindgen::prelude::*;

use super::Board;

#[wasm_bindgen]
struct Game(chess::Game);

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(ches::Game::new())
    }
}
