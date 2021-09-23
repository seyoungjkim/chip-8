extern crate minifb;
extern crate rand;
use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs;

mod cpu;

const SCALE: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();

    // Load rom data into emulator
    let rom_file_path = &args[1];
    print!("Playing rom {}\n", rom_file_path);
    let mut cpu = cpu::Cpu::new();
    let rom_data = fs::read(rom_file_path).expect("Error reading file");
    cpu.load_rom(&rom_data);

    // Create window
    const WINDOW_WIDTH: usize = cpu::DISPLAY_WIDTH * SCALE;
    const WINDOW_HEIGHT: usize = cpu::DISPLAY_HEIGHT * SCALE;
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let mut window = Window::new(
        "Chip-8",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.run_loop();

        for (index, is_on) in cpu.display.iter().enumerate() {
            if *is_on {
                for i in index * SCALE..index * SCALE + SCALE * SCALE {
                    buffer[i] = 0xFFFFFF;
                }
            } else {
                for i in index * SCALE..index * SCALE + SCALE * SCALE {
                    buffer[i] = 0;
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails.
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
