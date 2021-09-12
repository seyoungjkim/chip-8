extern crate rand;
use std::env;

mod cpu;

fn main() {
    let args: Vec<_> = env::args().collect();
    let rom_filename = &args[1];
    print!("Loading rom {}", rom_filename);

    let mut cpu = cpu::Cpu::new();

    cpu.run();
}
