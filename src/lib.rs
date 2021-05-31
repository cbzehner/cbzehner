extern crate web_sys;

use wasm_bindgen::prelude::*;

mod action;
mod event;
mod game;
mod item;
mod object;
mod obstacle;
mod player;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Game {}

#[wasm_bindgen]
impl Game {
    pub fn initialize() -> Self {
        utils::set_panic_hook();
        Self::new()
    }

    pub fn send_command(&mut self, input: &str) -> String {
        // Simply echo the input back for now
        log!("Recieved: > {}", &input);
        input.to_string()
    }

    fn new() -> Self {
        Self {}
    }
}
