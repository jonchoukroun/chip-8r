use std::thread;
use std::time::{Instant, Duration};

use crate::bus::Bus;
use crate::constants::{
    BIT_MASK,
    DISPLAY_HEIGHT,
    DISPLAY_WIDTH,
    FLAG_REGISTER,
    SPRITE_WIDTH,
    FONT_HEIGHT,
    FONT_RAM_START,
    MICROS_PER_FRAME, MICROS_PER_CYCLE
};
use crate::error::{Error, ErrorType};
use crate::registers::Registers;

pub struct Cpu {
    bus: Bus,
    registers: Registers,
    frame_buffer: FrameBuffer,
    opcode: u16,
    halted: bool,
    fps_timer: Instant,
}

pub type FrameBuffer = [u8; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize];
    
impl Cpu {
    pub fn new() -> Result<Cpu, String> {
        let mut bus = Bus::new()?;
        bus.init_ram();
        if bus.load_rom().is_err() {
            return Err(String::from("Failed to load ROM"));
        }

        let buffer: FrameBuffer = [
            0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize
        ];

        Ok(Cpu {
            bus,
            registers: Registers::new(),
            frame_buffer: buffer,
            opcode: 0x0000,
            halted: false,
            fps_timer: Instant::now(),
        })
    }

    pub fn run(&mut self) -> bool {
        loop {
            let cycle_timer = Instant::now();

            if !self.bus.handle_input() { break; }

            if self.halted {
                if let Some(key) = self.bus.get_keyup() {
                    self.registers.v[get_x(self.opcode)] = key;
                    self.halted = false;
                }
            } else {
                if let Some(_e) = self.fetch() {
                    return false;
                };
                self.execute();
            }

            if self.fps_timer.elapsed().as_micros() >= MICROS_PER_FRAME {
                self.bus.render(&self.frame_buffer);
                
                self.decrement_timers();
                self.fps_timer = Instant::now();
            }

            self.handle_audio();

            let diff = MICROS_PER_CYCLE - cycle_timer.elapsed().as_micros() as f32;
            if diff > 0.0 {
                thread::sleep(Duration::from_micros(diff as u64));
            }
        }
    
        true
    }

    fn fetch(&mut self) -> Option<Error> {
        if self.registers.pc < 0x200 || self.registers.pc >= 0xffe {
            return Some(Error::new(ErrorType::InaccessibleMemoryAddress))
        }
        let high = self.bus.read_byte(self.registers.pc) as u16;
        self.registers.pc += 1;
        let low = self.bus.read_byte(self.registers.pc) as u16;
        self.registers.pc += 1;

        self.opcode = (high << 8) | low;

        None
    }

    fn execute(&mut self) -> Option<Error> {
        match (self.opcode & 0xf000) >> 12 {
            0x0 => { self.opcode_0(self.opcode); },
            0x1 => { self.opcode_1(self.opcode); },
            0x2 => { self.opcode_2(self.opcode); },
            0x3 => { self.opcode_3(self.opcode); },
            0x4 => { self.opcode_4(self.opcode); },
            0x5 => { self.opcode_5(self.opcode); },
            0x6 => { self.opcode_6(self.opcode); },
            0x7 => { self.opcode_7(self.opcode); },
            0x8 => { self.opcode_8(self.opcode); },
            0x9 => { self.opcode_9(self.opcode); },
            0xa => { self.opcode_a(self.opcode); },
            0xb => { self.opcode_b(self.opcode); },
            0xc => { self.opcode_c(self.opcode); },
            0xd => { self.opcode_d(self.opcode); },
            0xe => { self.opcode_e(self.opcode); },
            0xf => { self.opcode_f(self.opcode); },
            _ => { Error::new(ErrorType::InvalidOpcode); }
            
        };
    
        None
    }

    fn decrement_timers(&mut self) {
        if self.registers.dt > 0 { self.registers.dt -= 1; }
        if self.registers.st > 0 { self.registers.st -= 1; }
    }

    fn handle_audio(&mut self) {
        if self.registers.st > 0 { self.bus.play_audio(); }
        else { self.bus.stop_audio(); }
    }
}

// Instructions
impl Cpu {
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
            _ => ()
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
        let x = get_x(opcode) as u8;
        let value = self.registers.v[x as usize];
        self.registers.v[x as usize] = value.wrapping_add(get_kk(opcode));
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
                if x as u16 + y as u16 > 0xff { self.registers.v[FLAG_REGISTER] = 1; }
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
            // SHR Vx, Vy
            0x6 => {
                let y = self.registers.v[get_y(opcode)];
                self.registers.v[get_x(opcode)] = y >> 1;
                self.registers.v[FLAG_REGISTER] = y & 1;
            },
            // SUB Vy, Vx
            0x7 => {
                let x = self.registers.v[get_x(opcode)];
                let y = self.registers.v[get_y(opcode)];
                self.registers.v[get_x(opcode)] = y.wrapping_sub(x);
                if y > x { self.registers.v[FLAG_REGISTER] = 1; }
                else { self.registers.v[FLAG_REGISTER] = 0; }
            },
            // SHL Vx, Vy
            0xe => {
                let y = self.registers.v[get_y(opcode)];
                self.registers.v[get_x(opcode)] = y << 1;
                self.registers.v[FLAG_REGISTER] = (y & 0b10000000) >> 7;
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

        self.registers.v[FLAG_REGISTER] = 0;

        for row in 0..get_n(opcode) {
            let sprite = self.bus.read_byte(i + row as u16);
            for col in 0..(SPRITE_WIDTH) {
                let pixel_x = x + col;
                if pixel_x >= DISPLAY_WIDTH as u8 { break; };

                let pixel_y = y + row;
                if pixel_y >= DISPLAY_HEIGHT as u8 { break; };

                let pixel_idx = 
                    pixel_y as usize * DISPLAY_WIDTH as usize + pixel_x as usize;
                let current_pixel = self.frame_buffer[pixel_idx];
                let new_pixel = (sprite & (BIT_MASK >> col)) >>
                    ((SPRITE_WIDTH - 1) - col);
                self.frame_buffer[pixel_idx] ^= new_pixel;
                if current_pixel == 1 && self.frame_buffer[pixel_idx] == 1 {
                    self.registers.v[FLAG_REGISTER] = 1;
                }
            }
        }
    }

    fn opcode_e(&mut self, opcode: u16) {
        let x = self.registers.v[get_x(opcode)];
        match get_kk(opcode) {
            // SKP Vx
            0x9e => {
                if self.bus.is_pressed(x) {
                    self.registers.pc += 2;
                }
            },
            // SKNP Vx
            0xa1 => {
                if !self.bus.is_pressed(x) {
                    self.registers.pc += 2;
                }
            },
            _ => println!("Invalid opcode {:#X}", opcode),
        }
    }

    fn opcode_f(&mut self, opcode: u16) {
        match get_kk(opcode) {
            // LD Vx, DT
            0x07 => {
                self.registers.v[get_x(opcode)] = self.registers.dt;
            },
            // LD Vx, K
            0x0a => {
                self.halted = true;
            },
            // LD DT, Vx
            0x15 => {
                self.registers.dt = self.registers.v[get_x(opcode)];
            },
            // LD ST, Vx
            0x18 => {
                self.registers.st = self.registers.v[get_x(opcode)];
            },
            // ADD I, Vx
            0x1e => {
                self.registers.i += self.registers.v[get_x(opcode)] as u16;
            },
            // LD F, Vx
            0x29 => {
                let x = self.registers.v[get_x(opcode)] & 0xf;
                self.registers.i = x as u16 
                    * FONT_HEIGHT as u16
                    + FONT_RAM_START as u16;
            },
            // LD BCD, Vx
            0x33 => {
                let i = self.registers.i as usize;
                let x = self.registers.v[get_x(opcode)];
                self.bus.write_byte(i, x / 100);
                self.bus.write_byte(i + 1, x % 100 / 10);
                self.bus.write_byte(i + 2, x % 10);
            },
            // LD [I], Vx
            0x55 => {
                let i = self.registers.i as usize;
                for j in 0..=get_x(opcode) {
                    self.bus.write_byte(i + j, self.registers.v[j]);
                }
            },
            // LD Vx, [I]
            0x65 => {
                let i = self.registers.i;
                for j in 0..=get_x(opcode) {
                    self.registers.v[j] = self.bus.read_byte(i + j as u16);
                }
            },
            _ => {
                println!("Invalid opcode {:#X}", opcode);
            }

        }
    }
}

fn get_x(opcode: u16) -> usize { ((opcode & 0x0f00) >> 8) as usize }

fn get_y(opcode: u16) -> usize { ((opcode & 0x00f0) >> 4) as usize }

fn get_n(opcode: u16) -> u8 { (opcode & 0x000f) as u8 }

fn get_kk(opcode: u16) -> u8 { (opcode & 0x00ff) as u8 }

fn get_nnn(opcode: u16) -> u16 { opcode & 0x0fff }