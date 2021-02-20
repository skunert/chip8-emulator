pub mod chip8;

extern crate sdl2;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];
    let mut chip8 = chip8::Emulator::initialize();
    chip8.run_file(cartridge_filename).expect("Unable to start emulation.");
}
