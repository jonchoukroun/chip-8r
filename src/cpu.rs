use crate::bus::Bus;
use crate::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::error::{Error, ErrorType};
use crate::registers::Registers;

pub struct CPU {
    bus: Bus,
    registers: Registers,
    frame_buffer: FrameBuffer,
}

pub type FrameBuffer = [u8; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize];
    
impl CPU {
    pub fn new() -> CPU {
        let mut buffer: FrameBuffer = [0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize];
        for i in 0..buffer.len() {
            if (i + i) % 3 == 0 { buffer[i] = 1 };
        }
        return CPU {
            bus: Bus::new(),
            registers: Registers::new(),
            frame_buffer: buffer,
        }
    }

    pub fn cycle(&mut self) -> Result<(), Error> {
        let opcode = match self.fetch() {
            Ok(instruction) => instruction,
            Err(e) => return Err(e),
        };

        match (opcode & 0xf000) >> 12 {
            0x0 => self.opcode_0(opcode),
            0x1 => self.opcode_1(opcode),
            0x6 => self.opcode_6(opcode),
            0x7 => self.opcode_7(opcode),
            0xa => self.opcode_a(opcode),
            0xd => self.opcode_d(opcode),
            _ => println!("Other"),
        }

        Ok(())
    }

    pub fn frame_buffer(&self) -> FrameBuffer {
        return self.frame_buffer;
    }

    fn fetch(&mut self) -> Result<u16, Error> {
        if self.registers.pc < 0x200 || self.registers.pc >= 0xffe {
            return Err(Error::new(ErrorType::InaccessibleMemoryAddress))
        }
        let high = self.bus.read_byte(self.registers.pc) as u16;
        self.registers.pc += 1;
        let low = self.bus.read_byte(self.registers.pc) as u16;
        self.registers.pc += 1;

        let opcode = (high << 8) | low;

        println!(
            "CPU fetch | opcode = {}, PC = {}",
            opcode, self.registers.pc);

        return Ok(opcode);
    }
}

// Instructions
impl CPU {
    fn opcode_0(&mut self, opcode: u16) {
        match opcode {
            // CLS
            0x00e0 => {
                self.frame_buffer = [0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize];
            },
            // RET
            0x00ee => {
                self.registers.pc = self.registers.stack[self.registers.sp as usize];
                self.registers.sp -= 1;
            }
            // SYS
            nibble => println!("sysjmp => {}", nibble),
        }
    }

    // JMP
    fn opcode_1(&mut self, opcode: u16) {
        self.registers.pc = opcode & 0x0fff;
    }

    // LD Vx
    fn opcode_6(&mut self, opcode: u16) {
        let x = (opcode & 0x0f00) >> 8;
        let kk = opcode & 0x00ff;
        self.registers.v[x as usize] = kk as u8;
    }

    // ADD Vx
    fn opcode_7(&mut self, opcode: u16) {
        let x = (opcode & 0x0f00) >> 8;
        let kk = opcode & 0x00ff;
        self.registers.v[x as usize] += kk as u8;
    }

    // LD I
    fn opcode_a(&mut self, opcode: u16) {
        self.registers.i = opcode & 0x0fff;
    }

    // DRW
    fn opcode_d(&mut self, opcode: u16) {
        let x = (opcode & 0x0f00) >> 8;
        let y = (opcode & 0x000f0) >> 4;
    }
}