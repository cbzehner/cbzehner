//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate text_adventure;
use text_adventure::Game;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn input() {
    let input = "Hello, World!";
    let mut game = Game::initialize();
    assert_eq!(game.send_command(input), input);
}
