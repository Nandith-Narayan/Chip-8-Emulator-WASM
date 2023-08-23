mod utils;
mod cpu;

use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;

use crate::cpu::CPU;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref CHIP8: CPU = cpu::init();
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert(format!("Hello, World! {}", CHIP8.memory.len()).as_str());
}
