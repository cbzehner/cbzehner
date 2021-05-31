mod action;
mod event;
mod game;
mod item;
mod object;
mod obstacle;
mod player;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {}

#[wasm_bindgen]
impl Game {
    pub fn initialize() -> Self {
        Self::new()
    }

    pub fn send_command(&mut self, input: &str) -> String {
        // Simply echo the input back for now
        input.to_string()
    }

    fn new() -> Self {
        Self {}
    }
}
