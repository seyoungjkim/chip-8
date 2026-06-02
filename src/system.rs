extern crate minifb;
extern crate rand;
use std::cell::RefCell;
use std::rc::Rc;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::panic;
use wasm_bindgen::prelude::{Closure};
use wasm_bindgen::JsCast;

mod cpu;

const SCALE: usize = 20;
const WINDOW_WIDTH: usize = cpu::DISPLAY_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = cpu::DISPLAY_HEIGHT * SCALE;

pub struct Chip8 {
    cpu: cpu::Cpu,
    buffer: Vec<u32>,
    window: Window,
    rom_loaded: bool,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            cpu: cpu::Cpu::new(),
            buffer: vec![0; WINDOW_WIDTH * WINDOW_HEIGHT],
            rom_loaded: false,
            window: Window::new(
                "CHIP-8",
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            }),
        }
    }

    fn run_game_loop(&mut self) {
        if !self.rom_loaded {
            self.window.update_with_buffer(&self.buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
            return
        }
        // get keyboard input changes
        for key in self.window.get_keys_pressed(KeyRepeat::No) {
            match map_key(key) {
                Some(i) => self.cpu.press_key(i),
                None => (),
            }
        }
        for key in self.window.get_keys_released() {
            match map_key(key) {
                Some(i) => self.cpu.release_key(i),
                None => (),
            }
        };

        // run cpu
        self.cpu.run_loop();

        // update display buffer
        for (index, is_on) in self.cpu.display.iter().enumerate() {
            let x = (index % cpu::DISPLAY_WIDTH) as usize;
            let y = (index / cpu::DISPLAY_WIDTH) as usize;
            let x_coord = x * SCALE;
            let y_coord = y * SCALE;
            for i in x_coord..x_coord + SCALE {
                for j in y_coord..y_coord + SCALE {
                    // High byte must be 0xFF: desktop (0xXRGB) ignores it, but WASM passes
                    // bytes directly to ImageData as RGBA, so alpha=0 makes all pixels transparent.
                    // TODO: understand this better.
                    self.buffer[i + WINDOW_WIDTH * j] = if *is_on { 0xFF_FF_FF_FF } else { 0xFF_00_00_00 };
                }
            }
        }
        self.window
            .update_with_buffer(&self.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap(); // exit on failure
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_chip_8(rom_data: Vec<u8>) {
    let mut chip_8 = Chip8::new();
    chip_8.cpu.load_rom(&rom_data);
    chip_8.window.set_target_fps(60);
    while chip_8.window.is_open() && !chip_8.window.is_key_down(Key::Escape) {
        chip_8.run_game_loop();
    }
}

thread_local! {
    static CHIP8: RefCell<Option<Chip8>> = RefCell::new(None);
}

// See https://github.com/dc740/minifb-async-examples/blob/main/web_app/src/web_setup/mod.rs
pub fn run_chip_8_wasm() {
    // Initialize interpreter
    let mut chip_8 = Chip8::new();
    chip_8.window.set_target_fps(60);
    CHIP8.with(|c| *c.borrow_mut() = Some(chip_8));

    // create the closure for updating and rendering the game
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        CHIP8.with(|c| {
            if let Some(chip_8) = c.borrow_mut().as_mut() {
                chip_8.run_game_loop();
            }
        });
        // schedule this closure for running again at next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() + 'static>));

    // start the animation loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window().expect("no global `window` exists")
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

pub fn load_rom_data(rom_data: Vec<u8>) {
    CHIP8.with(|c| {
          if let Some(chip_8) = c.borrow_mut().as_mut() {
              chip_8.rom_loaded = true;
              chip_8.cpu = cpu::Cpu::new();
              chip_8.cpu.load_rom(&rom_data);
          }
    });
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
