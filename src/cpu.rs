const STACK_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Cpu {
    // Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
    memory: [u8; MEMORY_SIZE],
    // Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    // A program counter, often called just “PC”, which points at the current instruction in memory
    pc: u8,
    // One 16-bit index register called “I” which is used to point at locations in memory
    i: u16,
    // A stack for 16-bit addresses, which is used to call subroutines/functions and return from them
    stack: [u16; STACK_SIZE],
    // An 8-bit delay timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
    delay_timer: u8,
    // An 8-bit sound timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
    sound_timer: u8,
    // 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through `VF
    // VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag
    registers: [u8; NUM_REGISTERS]
}

impl Cpu {
    pub fn new() -> Cpu {
        // TODO: initialize correct values
        Cpu {
            memory: [0; MEMORY_SIZE],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            pc: 0,
            i: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; NUM_REGISTERS]
        }
    }
}