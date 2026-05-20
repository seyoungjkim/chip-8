use wasm_bindgen::prelude::*;

mod system;

#[wasm_bindgen]
pub fn run_chip_8(rom_file_path: &str) {
    // TODO: get rom data
    let rom_data: Vec<u8> = Vec::new();
    system::run_chip_8(rom_data);
}
