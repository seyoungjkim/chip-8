use std::env;
use std::fs;

mod system;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Load rom data into emulator
    if args.len() < 2 {
        eprintln!("Please specify game file");
        return
    }
    let rom_file_path = &args[1];
    println!("Playing rom {}", rom_file_path);
    let rom_data = fs::read(rom_file_path).expect("Error reading file");
    system::initialize_and_run_chip_8(rom_data);
}
