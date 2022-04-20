# CHIP-8 Interpreter

This project is (yet another) Rust CHIP-8 Interpreter. It's my first project exploring the emulation space as well as my first Rust project.

For the frontend, I used the [minifb](https://github.com/emoon/rust_minifb) crate to display a window.

## Usage
Run `cargo run $PROGRAM_FILENAME` to run a Chip-8 program. On Windows, make sure [C++ build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) are installed to run Rust programs.

## Future Work
* Display in the browser
* Port to Android

## Resources Used
* https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
* https://aquova.net/chip8/chip8.pdf
* https://github.com/corax89/chip8-test-rom
* https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html
