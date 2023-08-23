mod utils;
mod cpu;

use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use js_sys::Uint8Array;

use crate::cpu::CPU;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref CHIP8: Mutex<CPU> = Mutex::new(cpu::init());
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(rom: &Uint8Array, len: i32) {

    let mut chip8 = CHIP8.lock().unwrap();
    chip8.memory[0]=2;

    alert(format!("Hello, World! {}", len).as_str());

}
