use crate::bus::Bus;
use crate::constants::{
    BIT_MASK, DISPLAY_HEIGHT, DISPLAY_WIDTH, FLAG_REGISTER, SPRITE_WIDTH
};
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
        let buffer: FrameBuffer = [
            0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize
        ];
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
            _ => println!("Undefined opcode: {:#X}", opcode),
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
                println!("CLS");
            },
            // RET
            0x00ee => {
                self.registers.pc = self.registers.stack[self.registers.sp as usize];
                self.registers.sp -= 1;
                println!("RET");
            }
            // SYS
            nibble => println!("sysjmp => {}", nibble),
        }
    }

    // JMP
    fn opcode_1(&mut self, opcode: u16) {
        self.registers.pc = opcode & 0x0fff;
        println!("JMP, {:#X}", self.registers.pc);
    }

    // LD Vx
    fn opcode_6(&mut self, opcode: u16) {
        let x = (opcode & 0x0f00) >> 8;
        let kk = opcode & 0x00ff;
        self.registers.v[x as usize] = kk as u8;
        println!("LD v[{:#X}], {:#X}", x, kk);
    }

    // ADD Vx
    fn opcode_7(&mut self, opcode: u16) {
        let x = (opcode & 0x0f00) >> 8;
        let kk = opcode & 0x00ff;
        self.registers.v[x as usize] += kk as u8;
        println!("ADD v[{:#X}], {:#X}", x, kk);
    }

    // LD I
    fn opcode_a(&mut self, opcode: u16) {
        self.registers.i = opcode & 0x0fff;
        println!("LD I, {:#X}", self.registers.i);
    }

    // DRW
    fn opcode_d(&mut self, opcode: u16) {
        let x_reg = (opcode & 0x0f00) >> 8;
        let y_reg = (opcode & 0x00f0) >> 4;
        let n = opcode & 0x000f;

        let x = self.registers.v[x_reg as usize] % DISPLAY_WIDTH as u8;
        let y = self.registers.v[y_reg as usize] % DISPLAY_HEIGHT as u8;
        let i = self.registers.i;

        println!("DRW {:#X}", opcode);

        self.registers.v[FLAG_REGISTER] = 0;

        for row in 0..n {
            let sprite = self.bus.read_byte(i + row);
            for col in 0..(SPRITE_WIDTH as u16) {
                let pixel_x = x as u16 + col;
                if pixel_x >= DISPLAY_WIDTH as u16 { break; };

                let pixel_y = y as u16 + row;
                if pixel_y >= DISPLAY_HEIGHT as u16 { break; };

                let pixel_idx: usize = (
                    pixel_y * DISPLAY_WIDTH as u16 + pixel_x
                ).into();
                let current_pixel = self.frame_buffer[pixel_idx];
                let new_pixel = (sprite & (BIT_MASK >> col)) >>
                    (SPRITE_WIDTH - 1) - col as u8;
                self.frame_buffer[pixel_idx] ^= new_pixel;
                if current_pixel == 1 && self.frame_buffer[pixel_idx] == 1 {
                    self.registers.v[FLAG_REGISTER] = 1;
                }
            }
        }
    }
}