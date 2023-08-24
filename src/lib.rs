mod utils;
mod cpu;

use std::panic;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use js_sys::Boolean;
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
pub fn initialize_rom(rom: Vec<u8>) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut chip8 = CHIP8.lock().unwrap();
    chip8.init();
    // Load ROM data into memory
    for i in 0..rom.len(){
        chip8.memory[i+512] = rom[i];
    }

    //alert(format!("Hello, World! {:?}", rom.len()).as_str());
}

#[wasm_bindgen]
pub fn run() -> Vec<Boolean>{
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut chip8 = CHIP8.lock().unwrap();

    for _ in 0..1_000{
        chip8.run_cycle();
    }

    let mut display = vec![];

    for i in 0..(64*32){
        display.push(Boolean::from(chip8.display[i]));
    }

    return display;

    //alert(format!("Hello, World! {:?}", rom.len()).as_str());
}