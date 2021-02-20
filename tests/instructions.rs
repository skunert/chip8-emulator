extern crate chip8;

#[cfg(test)]
mod tests {


    use chip8::chip8::cpu::Cpu;
    #[test]
    fn jp_addr() {
        let mut emu = Cpu::initialize();
        emu.cpu_step(0x1111);
        assert_eq!(emu.pc, 0x111);
        emu.cpu_step(0x1001);
        assert_eq!(emu.pc, 0x1);
    }

    #[test]
    fn call_addr() {
        let mut emu = Cpu::initialize();
        emu.cpu_step(0x2111);
        assert_eq!(emu.sp, 1);
        assert_eq!(emu.stack[0], 0x0usize);
        assert_eq!(emu.stack[1], 0x200usize);
    }
    #[test]
    fn se_vx_byte_no_skip() {
        let mut emu = Cpu::initialize();
        emu.cpu_step(0x3153);
        assert_eq!(emu.pc, 0x202);
    }

    #[test]
    fn se_vx_byte_skip() {
        let mut emu = Cpu::initialize();
        emu.v[1] = 0x53;
        emu.cpu_step(0x3153);
        assert_eq!(emu.pc, 0x204);

        emu.v[8] = 0x3;
        emu.cpu_step(0x3803);
        assert_eq!(emu.pc, 0x208);
    }

    #[test]
    fn sne_vx_byte_skip() {
        let mut emu = Cpu::initialize();
        emu.cpu_step(0x4153);
        assert_eq!(emu.pc, 0x204);

        emu.cpu_step(0x4803);
        assert_eq!(emu.pc, 0x208);

        emu.v[0xD] = 0xC;
        emu.cpu_step(0x4D0C);
        assert_eq!(emu.pc, 0x20A);
    }

    #[test]
    fn se_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xC] = 0x1;
        emu.v[0xD] = 0xE;
        emu.v[0xE] = 0x1;
        emu.cpu_step(0x5CD0);
        assert_eq!(emu.pc, 0x202);

        emu.cpu_step(0x5CE0);
        assert_eq!(emu.pc, 0x206);
    }

    #[test]
    fn ld_vx_byte() {
        let mut emu = Cpu::initialize();

        emu.cpu_step(0x6CD0);
        assert_eq!(emu.v[0xC], 0xD0);

        emu.cpu_step(0x6EDF);
        assert_eq!(emu.v[0xE], 0xDF);
    }

    #[test]
    fn add_vx_byte() {
        let mut emu = Cpu::initialize();

        emu.v[0x1] = 0;
        emu.cpu_step(0x7113);
        assert_eq!(emu.v[0x1], 0x13);

        emu.cpu_step(0x7105);
        assert_eq!(emu.v[0x1], 0x18);

        emu.cpu_step(0x71A5);
        assert_eq!(emu.v[0x1], 189);
    }

    #[test]
    fn ld_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 4;
        emu.v[0xE] = 8;
        emu.cpu_step(0x8AE0);
        assert_eq!(emu.v[0xA], 0x8);

        emu.cpu_step(0x8FA0);
        assert_eq!(emu.v[0xF], 0x8);
    }

    #[test]
    fn or_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 0x1;
        emu.v[0xE] = 0;
        emu.cpu_step(0x8AE1);
        assert_eq!(emu.v[0xA], 0x1);

        emu.v[0xF] = 0xF;
        emu.cpu_step(0x80F1);
        assert_eq!(emu.v[0], 0xF);

        emu.v[0x1] = 0x8;
        emu.v[0x3] = 0x1;
        emu.cpu_step(0x8311);
        assert_eq!(emu.v[3], 0x9);
    }

    #[test]
    fn and_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 0x1;
        emu.v[0xE] = 0;
        emu.cpu_step(0x8AE2);
        assert_eq!(emu.v[0xA], 0x0);

        emu.v[0xA] = 0b0011;
        emu.v[0xE] = 0b1100;
        emu.cpu_step(0x8AE2);
        assert_eq!(emu.v[0xA], 0x0);

        emu.v[0xA] = 0b1011;
        emu.v[0xE] = 0b1100;
        emu.cpu_step(0x8AE2);
        assert_eq!(emu.v[0xA], 0b1000);
    }

    #[test]
    fn xor_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 0b1001;
        emu.v[0xE] = 0b0110;
        emu.cpu_step(0x8AE3);
        assert_eq!(emu.v[0xA], 0b1111);

        emu.v[0xA] = 0b1111;
        emu.v[0xE] = 0b0110;
        emu.cpu_step(0x8AE3);
        assert_eq!(emu.v[0xA], 0b1001);
    }

    #[test]
    fn add_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 0x1;
        emu.v[0xE] = 0x4;
        emu.cpu_step(0x8AE4);
        assert_eq!(emu.v[0xA], 0x5);

        emu.v[0xA] = 0xA;
        emu.v[0xE] = 0x1;
        emu.cpu_step(0x8AE4);
        assert_eq!(emu.v[0xA], 0xB);

        emu.v[0xA] = 0xFF;
        emu.v[0xE] = 0x1;
        emu.cpu_step(0x8AE4);
        assert_eq!(emu.v[0xA], 0x0);
        assert_eq!(emu.v[0xF], 0x1);

        emu.v[0xA] = 0xFF;
        emu.v[0xE] = 0x2;
        emu.cpu_step(0x8AE4);
        assert_eq!(emu.v[0xA], 0x1);
        assert_eq!(emu.v[0xF], 0x1);
    }

    #[test]
    fn sub_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 0x1;
        emu.v[0xE] = 0x1;
        emu.cpu_step(0x8AE5);
        assert_eq!(emu.v[0xA], 0x0);
        assert_eq!(emu.v[0xA], 0x0);

        emu.v[0xA] = 0x10;
        emu.v[0xE] = 0x1;
        emu.cpu_step(0x8AE5);
        assert_eq!(emu.v[0xA], 0xF);
        assert_eq!(emu.v[0xF], 0x1);

        emu.v[0xA] = 0x0;
        emu.v[0xE] = 0xFF;
        emu.cpu_step(0x8AE5);
        assert_eq!(emu.v[0xA], 0x1);
        assert_eq!(emu.v[0xF], 0x0);
    }

    #[test]
    fn shr_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 0x5;
        emu.cpu_step(0x8AE6);
        assert_eq!(emu.v[0xA], 0x2);
        assert_eq!(emu.v[0xF], 0x1);

        emu.v[0xA] = 0xE;
        emu.cpu_step(0x8AE6);
        assert_eq!(emu.v[0xA], 0x7);
        assert_eq!(emu.v[0xF], 0x0);

        emu.v[0xA] = 0xFF;
        emu.cpu_step(0x8AE6);
        assert_eq!(emu.v[0xA], 127);
        assert_eq!(emu.v[0xF], 0x1);
    }

    #[test]
    fn subn_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 0x1;
        emu.v[0xE] = 0x1;
        emu.cpu_step(0x8AE7);
        assert_eq!(emu.v[0xA], 0x0);
        assert_eq!(emu.v[0xA], 0x0);

        emu.v[0xE] = 0x10;
        emu.v[0xA] = 0x1;
        emu.cpu_step(0x8AE7);
        assert_eq!(emu.v[0xA], 0xF);
        assert_eq!(emu.v[0xF], 0x1);

        emu.v[0xE] = 0x0;
        emu.v[0xA] = 0xFF;
        emu.cpu_step(0x8AE7);
        assert_eq!(emu.v[0xA], 0x1);
        assert_eq!(emu.v[0xF], 0x0);
    }

    #[test]
    fn shl_vx_vy() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 0b1;
        emu.cpu_step(0x8AEE);
        assert_eq!(emu.v[0xA], 0b10);
        assert_eq!(emu.v[0xF], 0x0);

        emu.v[0xA] = 0b1000_0000;
        emu.cpu_step(0x8AEE);
        assert_eq!(emu.v[0xA], 0);
        assert_eq!(emu.v[0xF], 1);
    }

    #[test]
    fn sne_vx_vy() {
        let mut emu = Cpu::initialize();

        assert_eq!(emu.pc, 0x200);
        emu.v[0xA] = 1;
        emu.v[0xE] = 1;
        emu.cpu_step(0x9AE0);
        assert_eq!(emu.pc, 0x202);

        emu.v[0xA] = 8;
        emu.v[0xE] = 1;
        emu.cpu_step(0x9AE0);
        assert_eq!(emu.pc, 0x206);
    }
    #[test]
    fn ld_i_addr() {
        let mut emu = Cpu::initialize();

        emu.cpu_step(0xAAE9);
        assert_eq!(emu.i, 0xAE9);

        emu.cpu_step(0xA111);
        assert_eq!(emu.i, 0x111);
    }

    #[test]
    fn rnd_vx_byte() {
        let mut emu = Cpu::initialize();

        emu.v[0xA] = 1;
        emu.cpu_step(0xCA00);
        assert_eq!(emu.v[0xA], 0);
    }
}
