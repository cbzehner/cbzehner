use wasm_bindgen::prelude::*;

use crate::{log, utils};

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
