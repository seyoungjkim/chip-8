use std::vec::Vec;

const STACK_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const STARTING_ADDRESS: u16 = 0x200;
const NUM_KEYS: usize = 16;

pub struct Cpu {
    // Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
    memory: [u8; MEMORY_SIZE],
    // Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    // A program counter, often called just “PC”, which points at the current instruction in memory
    pc: u16,
    // One 16-bit index register called “I” which is used to point at locations in memory
    i: u16,
    // A stack for 16-bit addresses, which is used to call subroutines/functions and return from them
    stack: Vec<u16>,
    // An 8-bit delay timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
    delay_timer: u8,
    // An 8-bit sound timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
    sound_timer: u8,
    // 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through `VF
    // VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag
    registers: [u8; NUM_REGISTERS],
    keys: [bool; NUM_KEYS]
}

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

impl Cpu {
    pub fn new() -> Cpu {
        // TODO: initialize correct values
        let mut memory = [0; MEMORY_SIZE];
        memory[80..160].clone_from_slice(&FONT);
        Cpu {
            memory: memory,
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            pc: STARTING_ADDRESS,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; NUM_REGISTERS],
            keys: [false; NUM_KEYS],
        }
    }

    fn fetch(&mut self) {

    }

    fn decode_and_execute(&mut self, ins: u16) {
        match ins {
            // clear screen
            0x00E0 => {
                for pixel in self.display.iter_mut() {
                     *pixel = false; 
                }
            }
            // jump
            0x1000 ..= 0x1FFF => {
                let jump_address = ins & 0x0FFF;
                self.pc = jump_address;
            }
            // // set register VX
            // 0x6XNN => {

            // }
            // // set index register I
            // 0xANNN => {

            // }
            // // display/draw
            // 0xDXYN => {

            // }
            _ => {}
        }

    }

    pub fn run(&mut self) {

    }
}
