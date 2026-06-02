mod chip8;

#[cfg(not(target_arch = "wasm32"))]
pub struct TerminalInterpreter {
    chip8: chip8::Chip8,
}

#[cfg(not(target_arch = "wasm32"))]
impl TerminalInterpreter {
    pub fn new() -> TerminalInterpreter {
        TerminalInterpreter {
            chip8: chip8::Chip8::new()
        }
    }

    pub fn load_rom_data_and_run_chip_8(&mut self, rom_data: Vec<u8>) {
        self.chip8.load_rom_data(&rom_data);
        while self.chip8.is_running() {
            self.chip8.run_game_loop();
        }
    }
}

#[cfg(target_arch = "wasm32")]
use {
    std::cell::RefCell,
    std::rc::Rc,
    wasm_bindgen::prelude::*,
    wasm_bindgen::JsCast,
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WasmInterpreter {
    chip8: Rc<RefCell<chip8::Chip8>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmInterpreter {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmInterpreter {
        WasmInterpreter {
            chip8: Rc::new(RefCell::new(chip8::Chip8::new()))
        }
    }

    pub fn run_chip_8(&mut self) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let chip8 = self.chip8.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            chip8.borrow_mut().run_game_loop();
            if chip8.borrow_mut().is_running() {
                request_animation_frame(f.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut() + 'static>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    pub fn load_rom_data(&mut self, rom_data: Vec<u8>) {
        self.chip8.borrow_mut().load_rom_data(&rom_data);
    }
}

#[cfg(target_arch = "wasm32")]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
