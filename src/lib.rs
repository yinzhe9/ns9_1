#![feature(unboxed_closures)]

mod utils;
mod b2048;
mod c2048;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    rmw_utf8::init()
}

#[wasm_bindgen]
pub fn encode(input: &str) -> String {
    c2048::encode(input)
}

#[wasm_bindgen]
pub fn decode(input: &str) -> String {
    c2048::decode(input)
}