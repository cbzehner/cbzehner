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
