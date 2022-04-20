extern crate minifb;
extern crate rand;
use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs;

mod cpu;

const SCALE: usize = 20;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Load rom data into emulator
    if args.len() < 2 {
        print!("Please specify game file");
        return
    }
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
        "CHIP-8",
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
            let x = (index % cpu::DISPLAY_WIDTH) as usize;
            let y = (index / cpu::DISPLAY_WIDTH) as usize;
            let x_coord = x * SCALE;
            let y_coord = y * SCALE;
            for i in x_coord..x_coord + SCALE {
                for j in y_coord..y_coord + SCALE {
                    buffer[i + WINDOW_WIDTH * j] = if *is_on { 0xFFFFFF } else { 0 };
                }
            }
        }

        // Get keyboard input
        window.get_keys().map(|keys| {
            for k in keys {
                match map_key(k) {
                    Some(i) => cpu.press_key(true, i),
                    None => (),
                }
            }
        });
        window.get_keys_released().map(|keys| {
            for k in keys {
                match map_key(k) {
                    Some(i) => cpu.press_key(false, i),
                    None => (),
                }
            }
        });

        // Exit on failure
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn map_key(key: Key) -> Option<usize> {
    match key {
        Key::Key1 => Some(0),
        Key::Key2 => Some(1),
        Key::Key3 => Some(2),
        Key::Key4 => Some(3),
        Key::Q => Some(4),
        Key::W => Some(5),
        Key::E => Some(6),
        Key::R => Some(7),
        Key::A => Some(8),
        Key::S => Some(9),
        Key::D => Some(10),
        Key::F => Some(11),
        Key::Z => Some(12),
        Key::X => Some(13),
        Key::C => Some(14),
        Key::V => Some(15),
        _ => None,
    }
}
