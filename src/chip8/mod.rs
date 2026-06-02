use minifb::{Key, KeyRepeat, Window, WindowOptions};
mod cpu;

const SCALE: usize = 20;
const WINDOW_WIDTH: usize = cpu::DISPLAY_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = cpu::DISPLAY_HEIGHT * SCALE;
const CYCLES_PER_SECOND: u16 = 700;

pub struct Chip8 {
    cpu: cpu::Cpu,
    buffer: Vec<u32>,
    window: Window,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut window = Window::new("CHIP-8", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default())
            .expect("Failed to create window");
        window.set_target_fps(60);
        Chip8 {
            cpu: cpu::Cpu::new(),
            buffer: vec![0; WINDOW_WIDTH * WINDOW_HEIGHT],
            window,
        }
    }

    pub fn load_rom_data(&mut self, rom_data: &[u8]) {
        self.cpu = cpu::Cpu::new(); // reset CPU
        self.cpu.load_rom(rom_data);
    }

    pub fn run_game_loop(&mut self, seconds_since_last_frame: f64) {
        for key in self.window.get_keys_pressed(KeyRepeat::No) {
            if let Some(i) = map_key(key) {
                self.cpu.press_key(i);
            }
        }
        for key in self.window.get_keys_released() {
            if let Some(i) = map_key(key) {
                self.cpu.release_key(i);
            }
        }

        self.cpu.decrement_timers();
        let cycles = (CYCLES_PER_SECOND as f64 * seconds_since_last_frame).round() as u32;
        for _ in 0..cycles {
            self.cpu.step_cpu();
        }

        for (index, is_on) in self.cpu.display().iter().enumerate() {
            let x = (index % cpu::DISPLAY_WIDTH) as usize;
            let y = (index / cpu::DISPLAY_WIDTH) as usize;
            let x_coord = x * SCALE;
            let y_coord = y * SCALE;
            for i in x_coord..x_coord + SCALE {
                for j in y_coord..y_coord + SCALE {
                    self.buffer[i + WINDOW_WIDTH * j] =
                        if *is_on { 0xFF_FF_FF_FF } else { 0xFF_00_00_00 };
                }
            }
        }
        self.window
            .update_with_buffer(&self.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }

    pub fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}

fn map_key(key: Key) -> Option<usize> {
    match key {
        Key::Key1 => Some(1),
        Key::Key2 => Some(2),
        Key::Key3 => Some(3),
        Key::Key4 => Some(12),
        Key::Q => Some(4),
        Key::W => Some(5),
        Key::E => Some(6),
        Key::R => Some(13),
        Key::A => Some(7),
        Key::S => Some(8),
        Key::D => Some(9),
        Key::F => Some(14),
        Key::Z => Some(10),
        Key::X => Some(0),
        Key::C => Some(11),
        Key::V => Some(15),
        _ => None,
    }
}
