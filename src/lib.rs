use wasm_bindgen::prelude::*;

mod system;

#[wasm_bindgen]
pub fn run_chip_8(rom_data: Vec<u8>) {
    system::run_chip_8(rom_data);
}
