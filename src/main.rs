use std::env;
use std::fs;

mod chip8;
mod platform;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Load rom data into interpreter
    if args.len() < 2 {
        eprintln!("Please specify game file");
        return
    }
    let rom_file_path = &args[1];
    println!("Playing rom {}", rom_file_path);
    let rom_data = fs::read(rom_file_path).expect("Error reading file");
    let mut interpreter = platform::TerminalInterpreter::new();
    interpreter.run_chip_8(rom_data);
}
