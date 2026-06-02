use super::chip8;

#[cfg(not(target_arch = "wasm32"))]
pub struct TerminalInterpreter {
    interpreter: chip8::Chip8,
}

#[cfg(not(target_arch = "wasm32"))]
impl TerminalInterpreter {
    pub fn new() -> TerminalInterpreter {
        TerminalInterpreter {
            interpreter: chip8::Chip8::new()
        }
    }

    pub fn run_chip_8(&mut self, rom_data: Vec<u8>) {
        self.interpreter.load_rom_data(&rom_data);
        while self.interpreter.is_running() {
            self.interpreter.run_game_loop();
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
    interpreter: Rc<RefCell<chip8::Chip8>>,
    started: bool,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmInterpreter {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmInterpreter {
        WasmInterpreter {
            interpreter: Rc::new(RefCell::new(chip8::Chip8::new())),
            started: false,
        }
    }

    pub fn run_chip_8(&mut self, rom_data: Vec<u8>) {
        let interpreter = self.interpreter.clone();
        interpreter.borrow_mut().load_rom_data(&rom_data);

        if self.started { return; }
        self.started = true;

        // Start game loop
        fn request_animation_frame(f: &Closure<dyn FnMut()>) {
            web_sys::window()
                .expect("no global `window` exists")
                .request_animation_frame(f.as_ref().unchecked_ref())
                .expect("should register `requestAnimationFrame` OK");
        }

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            interpreter.borrow_mut().run_game_loop();
            if interpreter.borrow().is_running() {
                request_animation_frame(f.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut() + 'static>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
