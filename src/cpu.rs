extern crate sdl2;

use crate::bus::Bus;
use crate::constants::{
    BIT_MASK, DISPLAY_HEIGHT, DISPLAY_WIDTH, FLAG_REGISTER, SPRITE_WIDTH
};
use crate::display::Display;
use crate::error::{Error, ErrorType};
use crate::keyboard::Keyboard;
use crate::registers::Registers;

pub struct CPU {
    bus: Bus,
    registers: Registers,
    frame_buffer: FrameBuffer,
    display: Display,
    keyboard: Keyboard,
}

pub type FrameBuffer = [u8; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize];
    
impl CPU {
    pub fn new() -> Result<CPU, String> {
        let sdl_context = sdl2::init()?;

        let buffer: FrameBuffer = [
            0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize
        ];

        let display = match Display::new(&sdl_context) {
            Ok(display) => display,
            Err(e) => { return Err(e); },
        };
        let keyboard = match Keyboard::new(&sdl_context) {
            Ok(keyboard) => keyboard,
            Err(e) => { return Err(e); },
        };

        // let audio = Audio::new(&sdl_context);

        return Ok(CPU {
            bus: Bus::new(),
            registers: Registers::new(),
            frame_buffer: buffer,
            display,
            keyboard,
        });
    }

    pub fn handle_input(&mut self) -> bool {
        return self.keyboard.handle_input();
    }

    pub fn cycle(&mut self) -> Result<(), Error> {
        let opcode = match self.fetch() {
            Ok(instruction) => instruction,
            Err(e) => return Err(e),
        };

        match (opcode & 0xf000) >> 12 {
            0x0 => self.opcode_0(opcode),
            0x1 => self.opcode_1(opcode),
            0x2 => self.opcode_2(opcode),
            0x3 => self.opcode_3(opcode),
            0x4 => self.opcode_4(opcode),
            0x5 => self.opcode_5(opcode),
            0x6 => self.opcode_6(opcode),
            0x7 => self.opcode_7(opcode),
            0x8 => self.opcode_8(opcode),
            0x9 => self.opcode_9(opcode),
            0xa => self.opcode_a(opcode),
            0xb => self.opcode_b(opcode),
            0xc => self.opcode_c(opcode),
            0xd => self.opcode_d(opcode),
            _ => println!("Undefined opcode: {:#X}", opcode),
        }

        return Ok(());
    }

    pub fn render(&mut self) {
        self.display.render(self.frame_buffer);
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
        println!("CPU fetch {:#X}, PC now at {:#X}", opcode, self.registers.pc);

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
                self.registers.pc = self.registers.stack[self.registers.sp];
                self.registers.sp -= 1;
            }
            // SYS
            nibble => println!("sysjmp => {}", nibble),
        }
    }

    // JMP nnn
    fn opcode_1(&mut self, opcode: u16) {
        self.registers.pc = opcode & 0x0fff;
    }

    // CALL nnn
    fn opcode_2(&mut self, opcode: u16) {
        self.registers.sp += 1;
        self.registers.stack[self.registers.sp] = self.registers.pc;
        self.registers.pc = opcode & 0x0fff;
    }

    // SE Vx, kk
    fn opcode_3(&mut self, opcode: u16) {
        if self.registers.v[get_x(opcode)] == get_kk(opcode) {
            self.registers.pc += 2;
        }
    }

    // SNE Vx, kk
    fn opcode_4(&mut self, opcode: u16) {
        if self.registers.v[get_x(opcode)] != get_kk(opcode) {
            self.registers.pc += 2;
        }
    }

    // SE Vx, Vy
    fn opcode_5(&mut self, opcode: u16) {
        if self.registers.v[get_x(opcode)] == self.registers.v[get_y(opcode)] {
            self.registers.pc += 2;
        }
    }

    // LD Vx
    fn opcode_6(&mut self, opcode: u16) {
        self.registers.v[get_x(opcode)] = get_kk(opcode);
    }

    // ADD Vx
    fn opcode_7(&mut self, opcode: u16) {
        self.registers.v[get_x(opcode)] += get_kk(opcode);
    }

    // Bitwise, Carry Arithmetic Instructions
    fn opcode_8(&mut self, opcode: u16) {
        match opcode & 0x000f {
            // LD Vx, Vy
            0x0 => self.registers.v[get_x(opcode)] = self.registers.v[get_y(opcode)],
            // OR Vx, Yy
            0x1 => self.registers.v[get_x(opcode)] |= self.registers.v[get_y(opcode)],
            // AND Vx, Vy
            0x2 => self.registers.v[get_x(opcode)] &= self.registers.v[get_y(opcode)],
            // XOR Vx, Vy
            0x3 => self.registers.v[get_x(opcode)] ^= self.registers.v[get_y(opcode)],
            // ADC Vx, Vy
            0x4 => {
                let x = self.registers.v[get_x(opcode)];
                let y = self.registers.v[get_y(opcode)];
                self.registers.v[get_x(opcode)] = x.wrapping_add(y);
                if 0xff - x > y { self.registers.v[FLAG_REGISTER] = 1; }
                else { self.registers.v[FLAG_REGISTER] = 0; }
            },
            // SUB Vx, Vy
            0x5 => {
                let x = self.registers.v[get_x(opcode)];
                let y = self.registers.v[get_y(opcode)];
                self.registers.v[get_x(opcode)] = x.wrapping_sub(y);
                if x > y { self.registers.v[FLAG_REGISTER] = 1; }
                else { self.registers.v[FLAG_REGISTER] = 0; }
            },
            // SHR Vx
            0x6 => {
                let x = self.registers.v[get_x(opcode)];
                self.registers.v[FLAG_REGISTER] = x & 0b1;
                self.registers.v[get_x(opcode)] = x >> 1;
            },
            // SUB Vy, Vx
            0x7 => {
                let x = self.registers.v[get_x(opcode)];
                let y = self.registers.v[get_y(opcode)];
                self.registers.v[get_x(opcode)] = y.wrapping_sub(x);
                if y > x { self.registers.v[FLAG_REGISTER] = 1; }
                else { self.registers.v[FLAG_REGISTER] = 0; }
            },
            // SHL Vx
            0xe => {
                let x = self.registers.v[get_x(opcode)];
                self.registers.v[FLAG_REGISTER] = x & 0b10000000;
                self.registers.v[get_x(opcode)] = x << 1;
            },
            _ => println!("Invalid opcode {:#X}", opcode),
        }
    }

    fn opcode_9(&mut self, opcode: u16) {
        if self.registers.v[get_x(opcode)] != self.registers.v[get_y(opcode)] {
            self.registers.pc += 2;
        }
    }

    // LD I
    fn opcode_a(&mut self, opcode: u16) {
        self.registers.i = opcode & get_nnn(opcode);
    }

    // JP V0, nnn
    fn opcode_b(&mut self, opcode: u16) {
        self.registers.pc = self.registers.v[0] as u16 + get_nnn(opcode);
    }

    // RND Vx, kk
    fn opcode_c(&mut self, opcode: u16) {
        let rnd = rand::random::<u8>();
        self.registers.v[get_x(opcode)] = rnd & get_kk(opcode);
    }

    // DRW
    fn opcode_d(&mut self, opcode: u16) {
        let x = self.registers.v[get_x(opcode)] % DISPLAY_WIDTH as u8;
        let y = self.registers.v[get_y(opcode)] % DISPLAY_HEIGHT as u8;
        let i = self.registers.i;

        println!("DRW {:#X}", opcode);

        self.registers.v[FLAG_REGISTER] = 0;

        for row in 0..get_n(opcode) {
            let sprite = self.bus.read_byte(i + row as u16);
            for col in 0..(SPRITE_WIDTH as u8) {
                let pixel_x = x + col;
                if pixel_x >= DISPLAY_WIDTH as u8 { break; };

                let pixel_y = y + row;
                if pixel_y >= DISPLAY_HEIGHT as u8 { break; };

                let pixel_idx: usize = (
                    pixel_y as usize * DISPLAY_WIDTH as usize + pixel_x as usize
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

fn get_x(opcode: u16) -> usize {
    return ((opcode & 0x0f00) >> 8) as usize;
}

fn get_y(opcode: u16) -> usize {
    return ((opcode & 0x00f0) >> 4) as usize;
}

fn get_n(opcode: u16) -> u8 {
    return (opcode & 0x000f) as u8;
}

fn get_kk(opcode: u16) -> u8 {
    return (opcode & 0x00ff) as u8;
}

fn get_nnn(opcode: u16) -> u16 {
    return opcode & 0x0fff;
}