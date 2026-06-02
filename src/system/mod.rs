mod chip8;
use chip8::Chip8;

#[cfg(not(target_arch = "wasm32"))]
pub fn initialize_and_run_chip_8(rom_data: Vec<u8>) {
    let mut chip_8 = Chip8::new();
    chip_8.load_rom(&rom_data);
    while chip_8.is_running() {
        chip_8.run_game_loop();
    }
}

#[cfg(target_arch = "wasm32")]
use {
    std::cell::RefCell,
    std::rc::Rc,
    wasm_bindgen::prelude::Closure,
    wasm_bindgen::JsCast,
};

#[cfg(target_arch = "wasm32")]
thread_local! {
    static CHIP8: RefCell<Option<Chip8>> = RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
pub fn run_chip_8() {
    CHIP8.with(|c| *c.borrow_mut() = Some(Chip8::new()));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        CHIP8.with(|c| {
            if let Some(chip_8) = c.borrow_mut().as_mut() {
                chip_8.run_game_loop();
            }
        });
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() + 'static>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[cfg(target_arch = "wasm32")]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[cfg(target_arch = "wasm32")]
pub fn load_rom_data(rom_data: Vec<u8>) {
    CHIP8.with(|c| {
        if let Some(chip_8) = c.borrow_mut().as_mut() {
            chip_8.load_rom(&rom_data);
        }
    });
}
