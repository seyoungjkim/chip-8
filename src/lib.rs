#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod system;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_chip_8() {
    system::run_chip_8();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn load_rom_data(rom_data: Vec<u8>) {
    system::load_rom_data(rom_data);
}
