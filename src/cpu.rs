use std::vec::Vec;
use rand::Rng;

const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const NUM_KEYS: usize = 16;

const STARTING_ADDRESS: u16 = 0x200;
const FONT_SIZE: usize = 80;

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

const FONT: [u8; FONT_SIZE] = [
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

    pub fn run(&mut self) {
        // TODO: implement loop
        self.decrement_timers();
        let ins : u16 = self.fetch();
        self.decode_and_execute(ins);
    }

    fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            // TODO: beep
            self.sound_timer -= 1;
        }
    }

    fn fetch(&mut self) -> u16 {
        let first_byte = self.memory[self.pc as usize];
        let second_byte = self.memory[(self.pc) as usize];
        self.pc += 2;
        (first_byte as u16) << 8 | second_byte as u16
    }

    fn decode_and_execute(&mut self, ins: u16) {
        let opcode = (ins & 0xF000) >> 12;
        let x = ((ins & 0x0F00) >> 8) as usize;
        let y = ((ins & 0x00F0) >> 4) as usize;
        let n = ins & 0x000F;
        let nn = (ins & 0x0FF) as u8;
        let nnn = ins * 0x0FFF;

        match (opcode, x, y, n) {
            // clear screen
            (0, 0, 0xE, 0) => {
                for pixel in self.display.iter_mut() {
                    *pixel = false;
                }
            }
            // return from subroutine
            (0, 0, 0xE, 0xE) => {
                let return_address = self.stack.pop().unwrap();
                self.pc = return_address;
            }
            // jump to NNN
            (1, _, _, _) => {
                self.pc = nnn;
            }
            // call subroutine at NNN
            (2, _, _, _) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            // skip 1 instruction if VX == NN
            (3, _, _, _) => {
                if self.registers[x] == nn {
                    self.pc += 2;
                }
            }
            // skip 1 instruction if VX != NN
            (4, _, _, _) => {
                if self.registers[x] != nn {
                    self.pc += 2;
                }
            }
            // skip 1 instruction if VX == VY
            (5, _, _, 0) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            // skip 1 instruction if VX != VY
            (9, _, _, 0) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            // set register VX to NN
            (6, _, _, _) => {
                self.registers[x] = nn;
            }
            // add NN to register VX
            (7, _, _, _) => {
                self.registers[x] += nn;
            }
            // set VX to the value of VY
            (8, _, _, 0) => {
                self.registers[x] = self.registers[y];
            }
            // set VX = VX OR VY
            (8, _, _, 1) => {
                self.registers[x] = self.registers[x] | self.registers[y];
            }
            // set VX = VX AND VY
            (8, _, _, 2) => {
                self.registers[x] = self.registers[x] & self.registers[y];
            }
            // set VX = VX XOR VY
            (8, _, _, 3) => {
                self.registers[x] = self.registers[x] ^ self.registers[y];
            }
            // set VX = VX + VY
            (8, _, _, 4) => {
                let (new_vx_value, result_overflowed) =
                    self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = new_vx_value;
                self.registers[0xF] = if result_overflowed { 1 } else { 0 };
            }
            // set VX = VX - VY
            (8, _, _, 5) => {
                self.registers[x] = self.registers[x] - self.registers[y];
                self.registers[0xF] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
            }
            // shift VX 1 bit right
            (8, _, _, 6) => {
                self.registers[0xF] = self.registers[x] & 1;
                self.registers[x] >>= 1;
            }
            // set VX = VY - VX
            (8, _, _, 7) => {
                self.registers[x] = self.registers[y] - self.registers[x];
                self.registers[0xF] = if self.registers[y] > self.registers[x] { 1 } else { 0 };
            }
            // shift VX 1 bit left
            (8, _, _, 0xE) => {
                self.registers[0xF] = self.registers[x] >> 7;
                self.registers[x] <<= self.registers[x];
            }
            // set index register to NNN
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            // jump to NNN + V0
            (0xB, _, _, _) => {
                self.pc = nnn + (self.registers[0] as u16);
            }
            // set VX = random number & NN
            (0xC, _, _, _) => {
                let random_num: u8 = rand::thread_rng().gen();
                self.registers[x] = random_num & nn;
            }
            // display/draw
            (0xD, _, _, _) => {
                // TODO: implement display instruction
            }
            // skip if key corresponding to VX is pressed
            (0xE, _, 9, 0xE) => {
                let key = self.registers[x] as usize;
                if self.keys[key] {
                    self.pc += 2;
                }
            }
            // skip if key corresponding to VX is not pressed
            (0xE, _, 0xA, 1) => {
                let key = self.registers[x] as usize;
                if !self.keys[key] {
                    self.pc += 2;
                }
            }
            // set VX to value of delay timers
            (0xF, _, 0, 7) => {
                self.registers[x] = self.delay_timer;
            }
            // set delay timer to value of VX
            (0xF, _, 1, 5) => {
                self.delay_timer = self.registers[x];
            }
            // set sound timer to value of VX
            (0xF, _, 1, 8) => {
                self.sound_timer = self.registers[x];
            }
            // TODO: implement remaining instructions
            (0xF, _, 1, 0xE) => {

            }
            (0xF, _, 0, 0xA) => {

            }
            (0xF, _, 2, 9) => {

            }
            (0xF, _, 3, 3) => {

            }
            (0xF, _, 5, 5) => {

            }
            (0xF, _, 6, 5) => {
                
            }
            _ => return
        }

    }
}
