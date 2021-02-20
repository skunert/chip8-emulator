pub mod cpu;
pub mod display;
pub mod sound;

use display::Display;

use std::time::Duration;

pub struct Emulator {}

impl Emulator {
    pub fn initialize() -> Emulator {
        Emulator {}
    }

    pub fn run_file(&mut self, filepath: &str) -> Result<(), String> {
        let sdl_context = sdl2::init().expect("Unable to initialize sdl");
        let sound_controller = sound::build_sound_controller();
        let mut cpu = cpu::Cpu::initialize();
        let mut display = Display::new(&sdl_context);
        cpu.load_rom(filepath);

        loop {
            if cpu.check_key_events(&sdl_context) {
                break;
            };

            let step_result = cpu.step();
            if step_result.make_sound {
                sound_controller.play();
            } else {
                sound_controller.stop();
            }

            display.draw(step_result.graphics);
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 360));
        }
        Ok(())
    }
}
