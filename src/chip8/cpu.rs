use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs;
enum KeyActions {
    Quit,
    KeyUpDown(usize),
    None,
}

enum ProgramCounterAction {
    Skip,
    Advance,
    Wait,
    Jump(usize),
}

pub struct StepResult {
    pub graphics: [[u8; 64]; 32],
    pub make_sound: bool,
}

pub struct OpCode {
    pub x: usize,
    pub y: usize,
    pub n: usize,
    pub kk: u8,
    pub nnn: usize,
    pub nibbles: (u8, u8, u8, u8),
    pub raw: u16,
}

impl OpCode {
    pub fn from_u16(raw_code: u16) -> OpCode {
        let nibbles = (
            ((raw_code & 0xF000) >> 12) as u8,
            ((raw_code & 0x0F00) >> 8) as u8,
            ((raw_code & 0x00F0) >> 4) as u8,
            (raw_code & 0x000F) as u8,
        );
        OpCode {
            nibbles: (
                ((raw_code & 0xF000) >> 12) as u8,
                ((raw_code & 0x0F00) >> 8) as u8,
                ((raw_code & 0x00F0) >> 4) as u8,
                (raw_code & 0x000F) as u8,
            ),
            raw: raw_code,
            nnn: (raw_code & 0x0FFF) as usize,
            kk: (raw_code & 0x00FF) as u8,
            x: nibbles.1 as usize,
            y: nibbles.2 as usize,
            n: nibbles.3 as usize,
        }
    }
}

pub struct Cpu {
    rng: rand::rngs::ThreadRng,
    pub memory: Vec<u8>,
    pub v: Vec<u8>,
    pub pc: usize,
    pub i: u16,
    pub graphics: [[u8; 64]; 32],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: Vec<usize>,
    pub sp: usize,
    pub key: Vec<bool>,
    pub wait_key: bool,
}

impl Cpu {
    pub fn initialize() -> Cpu {
        let mut result = Cpu {
            rng: rand::thread_rng(),
            memory: vec![0; 4096],
            v: vec![0; 16],
            graphics: [[0; 64]; 32],
            pc: 0x200,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![0; 16],
            sp: 0,
            key: vec![false; 16],
            wait_key: false,
        };
        result.initialize_font_data();

        result
    }

    fn initialize_font_data(&mut self) {
        let font_array: Vec<u8> = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];
        for (font_value, memory_value) in font_array
            .as_slice()
            .iter()
            .zip(self.memory[0x0..].as_mut())
        {
            *memory_value = *font_value;
        }
    }

    pub fn load_rom(&mut self, filepath: &str) {
        let rom = fs::read(filepath).expect("Unable to read file.");
        let start_address: usize = 0x200;
        let end_address = start_address + rom.len();
        if end_address > self.memory.len() {
            panic!("Rom is larger than memory. Aborting.");
        }

        self.memory[start_address..end_address].copy_from_slice(&rom);
    }

    pub fn step(&mut self) -> StepResult {
        let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        self.cpu_step(opcode)
    }

    fn inst_00ee(&mut self) -> ProgramCounterAction {
        self.pc = self.stack[self.sp];
        self.sp -= 1;
        ProgramCounterAction::Advance
    }

    fn inst_1nnn(&mut self, opcode: OpCode) -> ProgramCounterAction {
        ProgramCounterAction::Jump(opcode.nnn)
    }

    fn inst_2nnn(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        ProgramCounterAction::Jump(opcode.nnn)
    }

    fn inst_3xkk(&mut self, opcode: OpCode) -> ProgramCounterAction {
        if self.v[opcode.x] == opcode.kk {
            ProgramCounterAction::Skip
        } else {
            ProgramCounterAction::Advance
        }
    }
    fn inst_4xkk(&mut self, opcode: OpCode) -> ProgramCounterAction {
        if self.v[opcode.x] != opcode.kk {
            ProgramCounterAction::Skip
        } else {
            ProgramCounterAction::Advance
        }
    }

    fn inst_5xy0(&mut self, opcode: OpCode) -> ProgramCounterAction {
        if self.v[opcode.x] == self.v[opcode.y] {
            ProgramCounterAction::Skip
        } else {
            ProgramCounterAction::Advance
        }
    }

    fn inst_6xkk(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[opcode.x] = opcode.kk;
        ProgramCounterAction::Advance
    }

    fn inst_7xkk(&mut self, opcode: OpCode) -> ProgramCounterAction {
        let register = opcode.x;
        self.v[register] = (self.v[register] as usize + opcode.kk as usize) as u8;
        ProgramCounterAction::Advance
    }

    fn inst_8xy0(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[opcode.x] = self.v[opcode.y];
        ProgramCounterAction::Advance
    }

    fn inst_8xy1(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[opcode.x] = self.v[opcode.x] | self.v[opcode.y];
        ProgramCounterAction::Advance
    }

    fn inst_8xy2(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[opcode.x] = self.v[opcode.x] & self.v[opcode.y];
        ProgramCounterAction::Advance
    }

    fn inst_8xy3(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[opcode.x] = self.v[opcode.x] ^ self.v[opcode.y];
        ProgramCounterAction::Advance
    }

    fn inst_8xy4(&mut self, opcode: OpCode) -> ProgramCounterAction {
        let result = (self.v[opcode.x] as u16) + (self.v[opcode.y] as u16);
        self.v[opcode.x] = (result & 0xFF) as u8;
        self.v[0xF] = if result > 255 { 1 } else { 0 };
        ProgramCounterAction::Advance
    }

    fn inst_8xy5(&mut self, opcode: OpCode) -> ProgramCounterAction {
        let result = (self.v[opcode.x] as i16) - (self.v[opcode.y] as i16);
        self.v[0xF] = if self.v[opcode.x] > self.v[opcode.y] {
            1
        } else {
            0
        };
        self.v[opcode.x] = (result & 0xFF) as u8;
        ProgramCounterAction::Advance
    }

    fn inst_8xy6(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[0xF] = if self.v[opcode.x] & 0x1 > 0 { 1 } else { 0 };
        self.v[opcode.x] = self.v[opcode.x] / 2;
        ProgramCounterAction::Advance
    }

    fn inst_8xy7(&mut self, opcode: OpCode) -> ProgramCounterAction {
        let result = (self.v[opcode.y] as i16) - (self.v[opcode.x] as i16);
        self.v[0xF] = if self.v[opcode.y] > self.v[opcode.x] {
            1
        } else {
            0
        };
        self.v[opcode.x] = (result & 0xFF) as u8;
        ProgramCounterAction::Advance
    }

    fn inst_8xye(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[0xF] = if self.v[opcode.x] & 0x80 > 0 { 1 } else { 0 };
        self.v[opcode.x] = self.v[opcode.x] << 1;
        ProgramCounterAction::Advance
    }

    fn inst_9xy0(&mut self, opcode: OpCode) -> ProgramCounterAction {
        if self.v[opcode.x] != self.v[opcode.y] {
            return ProgramCounterAction::Skip;
        }
        ProgramCounterAction::Advance
    }

    fn inst_annn(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.i = opcode.nnn as u16;
        ProgramCounterAction::Advance
    }

    fn inst_bnnn(&mut self, opcode: OpCode) -> ProgramCounterAction {
        ProgramCounterAction::Jump((opcode.nnn as u16 + self.v[0] as u16) as usize)
    }

    fn inst_cxkk(&mut self, opcode: OpCode) -> ProgramCounterAction {
        let random: u8 = self.rng.gen();
        self.v[opcode.x] = random & opcode.kk;
        ProgramCounterAction::Advance
    }

    fn inst_dxyn(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[0xF] = 0;
        for byte in 0..opcode.n {
            let y = (self.v[opcode.y] as usize + byte as usize) % 32;
            for bit in 0..8 {
                let x = (self.v[opcode.x] as usize + bit) % 64;
                let color = (self.memory[self.i as usize + byte as usize] >> (7 - bit)) & 1;
                self.v[0x0f] |= color & self.graphics[y][x];
                self.graphics[y][x] ^= color;
            }
        }
        ProgramCounterAction::Advance
    }

    fn inst_ex9e(&mut self, opcode: OpCode) -> ProgramCounterAction {
        if self.key[self.v[opcode.x] as usize] {
            return ProgramCounterAction::Skip;
        }
        ProgramCounterAction::Advance
    }

    fn inst_exa1(&mut self, opcode: OpCode) -> ProgramCounterAction {
        if !self.key[self.v[opcode.x] as usize] {
            return ProgramCounterAction::Skip;
        }
        ProgramCounterAction::Advance
    }

    fn inst_fx07(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.v[opcode.x] = self.delay_timer;
        ProgramCounterAction::Advance
    }

    fn inst_fx0a(&mut self, opcode: OpCode) -> ProgramCounterAction {
        if !self.wait_key {
            self.wait_key = true;
        } else {
            for (index, keypressed) in self.key.iter().enumerate() {
                if *keypressed {
                    self.wait_key = false;
                    self.v[opcode.x] = index as u8;
                    return ProgramCounterAction::Advance;
                }
            }
        }
        ProgramCounterAction::Wait
    }

    fn inst_fx15(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.delay_timer = self.v[opcode.x];
        ProgramCounterAction::Advance
    }

    fn inst_fx18(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.sound_timer = self.v[opcode.x];
        ProgramCounterAction::Advance
    }

    fn inst_fx1e(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.i = self.i + self.v[opcode.x] as u16;
        ProgramCounterAction::Advance
    }

    fn inst_fx29(&mut self, opcode: OpCode) -> ProgramCounterAction {
        let digit = self.v[opcode.x];
        self.i = digit as u16 * 5;
        ProgramCounterAction::Advance
    }

    fn inst_fx33(&mut self, opcode: OpCode) -> ProgramCounterAction {
        self.memory[self.i as usize] = self.v[opcode.x] / 100;
        self.memory[self.i as usize + 1] = (self.v[opcode.x] % 100) / 10;
        self.memory[self.i as usize + 2] = self.v[opcode.x] % 10;
        ProgramCounterAction::Advance
    }

    fn inst_fx55(&mut self, opcode: OpCode) -> ProgramCounterAction {
        for i in 0..=opcode.x {
            self.memory[self.i as usize + i] = self.v[i];
        }
        ProgramCounterAction::Advance
    }

    fn inst_fx65(&mut self, opcode: OpCode) -> ProgramCounterAction {
        for i in 0..=opcode.x {
            self.v[i] = self.memory[self.i as usize + i]
        }
        ProgramCounterAction::Advance
    }

    pub fn cpu_step(&mut self, opcode: u16) -> StepResult {
        let opcode = OpCode::from_u16(opcode);
        let pc_action = match opcode.nibbles {
            (0x0, 0x0, 0xE, 0x0) => ProgramCounterAction::Advance,
            (0x0, 0x0, 0xE, 0xE) => self.inst_00ee(),
            (0x0, _, _, _) => ProgramCounterAction::Advance,
            (0x1, _, _, _) => self.inst_1nnn(opcode),
            (0x2, _, _, _) => self.inst_2nnn(opcode),
            (0x3, _, _, _) => self.inst_3xkk(opcode),
            (0x4, _, _, _) => self.inst_4xkk(opcode),
            (0x5, _, _, 0) => self.inst_5xy0(opcode),
            (0x6, _, _, _) => self.inst_6xkk(opcode),
            (0x7, _, _, _) => self.inst_7xkk(opcode),
            (0x8, _, _, 0x0) => self.inst_8xy0(opcode),
            (0x8, _, _, 0x1) => self.inst_8xy1(opcode),
            (0x8, _, _, 0x2) => self.inst_8xy2(opcode),
            (0x8, _, _, 0x3) => self.inst_8xy3(opcode),
            (0x8, _, _, 0x4) => self.inst_8xy4(opcode),
            (0x8, _, _, 0x5) => self.inst_8xy5(opcode),
            (0x8, _, _, 0x6) => self.inst_8xy6(opcode),
            (0x8, _, _, 0x7) => self.inst_8xy7(opcode),
            (0x8, _, _, 0xE) => self.inst_8xye(opcode),
            (0x9, _, _, _) => self.inst_9xy0(opcode),
            (0xA, _, _, _) => self.inst_annn(opcode),
            (0xB, _, _, _) => self.inst_bnnn(opcode),
            (0xC, _, _, _) => self.inst_cxkk(opcode),
            (0xD, _, _, _) => self.inst_dxyn(opcode),
            (0xE, _, 0x9, 0xE) => self.inst_ex9e(opcode),
            (0xE, _, 0xA, 0x1) => self.inst_exa1(opcode),
            (0xF, _, 0x0, 0x7) => self.inst_fx07(opcode),
            (0xF, _, 0x0, 0xA) => self.inst_fx0a(opcode),
            (0xF, _, 0x1, 0x5) => self.inst_fx15(opcode),
            (0xF, _, 0x1, 0x8) => self.inst_fx18(opcode),
            (0xF, _, 0x1, 0xE) => self.inst_fx1e(opcode),
            (0xF, _, 0x2, 0x9) => self.inst_fx29(opcode),
            (0xF, _, 0x3, 0x3) => self.inst_fx33(opcode),
            (0xF, _, 0x5, 0x5) => self.inst_fx55(opcode),
            (0xF, _, 0x6, 0x5) => self.inst_fx65(opcode),
            (_, _, _, _) => {
                eprintln!("Unknown opcode");
                ProgramCounterAction::Advance
            }
        };

        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1
        }

        match pc_action {
            ProgramCounterAction::Skip => self.pc += 4,
            ProgramCounterAction::Advance => self.pc += 2,
            ProgramCounterAction::Jump(address) => self.pc = address,
            _ => {}
        }

        StepResult {
            graphics: self.graphics,
            make_sound: self.sound_timer != 0,
        }
    }
    pub fn check_key_events(&mut self, sdl_context: &sdl2::Sdl) -> bool {
        let mut event_pump = sdl_context
            .event_pump()
            .expect("Unable to poll events from sdl");
        for event in event_pump.poll_iter() {
            let key_state = match event {
                Event::KeyDown { .. } => true,
                _ => false,
            };

            let action = match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => KeyActions::Quit,
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => match code {
                    Keycode::Q => KeyActions::KeyUpDown(0xa),
                    Keycode::W => KeyActions::KeyUpDown(0xb),
                    Keycode::E => KeyActions::KeyUpDown(0xc),
                    Keycode::R => KeyActions::KeyUpDown(0xd),
                    Keycode::T => KeyActions::KeyUpDown(0xe),
                    Keycode::Y => KeyActions::KeyUpDown(0xf),
                    Keycode::U => KeyActions::KeyUpDown(0),
                    Keycode::I => KeyActions::KeyUpDown(1),
                    Keycode::A => KeyActions::KeyUpDown(2),
                    Keycode::S => KeyActions::KeyUpDown(3),
                    Keycode::D => KeyActions::KeyUpDown(4),
                    Keycode::F => KeyActions::KeyUpDown(5),
                    Keycode::G => KeyActions::KeyUpDown(6),
                    Keycode::H => KeyActions::KeyUpDown(7),
                    Keycode::J => KeyActions::KeyUpDown(8),
                    Keycode::K => KeyActions::KeyUpDown(9),
                    _ => KeyActions::None,
                },
                _ => KeyActions::None,
            };

            match action {
                KeyActions::Quit => {
                    return true;
                }
                KeyActions::KeyUpDown(address) => {
                    self.key[address] = key_state;
                }
                KeyActions::None => {}
            }
        }
        return false;
    }
}
