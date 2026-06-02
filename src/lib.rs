use wasm_bindgen::prelude::*;

mod system;

#[wasm_bindgen]
pub fn run_chip_8() {
    system::run_chip_8_wasm();
}

#[wasm_bindgen]
pub fn load_rom_data(rom_data: Vec<u8>) {
    system::load_rom_data(rom_data);
}
