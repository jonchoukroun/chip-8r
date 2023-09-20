extern crate sdl2;

use crate::bus::Bus;
use crate::constants::{
    BIT_MASK,
    DISPLAY_HEIGHT,
    DISPLAY_WIDTH,
    FLAG_REGISTER,
    SPRITE_WIDTH,
    FONT_HEIGHT,
    FONT_RAM_START
};
use crate::display::Display;
use crate::error::{Error, ErrorType};
use crate::GameState;
use crate::keyboard::{Keyboard, handle_input, is_pressed, to_hex};
use crate::registers::Registers;

pub struct CPU {
    bus: Bus,
    registers: Registers,
    frame_buffer: FrameBuffer,
    display: Display,
    keyboard: Keyboard,
    last_opcode: u16,
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
            last_opcode: 0x0000,
        });
    }

    pub fn handle_input(&mut self, state: &GameState) -> GameState {
        return handle_input(&mut self.keyboard, state);
    }

    pub fn check_input(&mut self, game_state: &mut GameState) {
        if *game_state != GameState::Paused { return };
        
        match self.keyboard.key_state.iter()
            .find_map(
                |(k, &v)| if v == true { Some(k) } else { None }
            ) {
                None => (),
                Some(scancode) => {
                    let x = get_x(self.last_opcode);
                    let kk = match to_hex(*scancode) {
                        Some(x) => x,
                        None => { return; },
                    };
                    self.registers.v[x as usize] = kk;
                    *game_state = GameState::Playing;
                }
            }
    }

    pub fn cycle(&mut self) -> Result<GameState, Error> {
        let opcode = match self.fetch() {
            Ok(instruction) => instruction,
            Err(e) => return Err(e),
        };

        let state = match (opcode & 0xf000) >> 12 {
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
            0xe => self.opcode_e(opcode),
            0xf => self.opcode_f(opcode),
            _ => {
                return Err(Error::new(ErrorType::InvalidOpcode));
            }
        };

        return Ok(state);
    }

    pub fn render(&mut self) {
        self.display.render(self.frame_buffer);
    }

    pub fn decrement_timers(&mut self) {
        if self.registers.dt > 0 { self.registers.dt -= 1; }
        if self.registers.st > 0 { self.registers.st -= 1; }
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
        self.last_opcode = opcode;
        println!("CPU fetch {:#X}, PC now at {:#X}", opcode, self.registers.pc);

        return Ok(opcode);
    }
}

// Instructions
impl CPU {
    fn opcode_0(&mut self, opcode: u16) -> GameState {
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

        return GameState::Playing;
    }

    // JMP nnn
    fn opcode_1(&mut self, opcode: u16) -> GameState {
        self.registers.pc = opcode & 0x0fff;
        return  GameState::Playing;
    }

    // CALL nnn
    fn opcode_2(&mut self, opcode: u16) -> GameState {
        self.registers.sp += 1;
        self.registers.stack[self.registers.sp] = self.registers.pc;
        self.registers.pc = opcode & 0x0fff;

        return GameState::Playing;
    }

    // SE Vx, kk
    fn opcode_3(&mut self, opcode: u16) -> GameState {
        if self.registers.v[get_x(opcode)] == get_kk(opcode) {
            self.registers.pc += 2;
        }
        return GameState::Playing;
    }

    // SNE Vx, kk
    fn opcode_4(&mut self, opcode: u16) -> GameState {
        if self.registers.v[get_x(opcode)] != get_kk(opcode) {
            self.registers.pc += 2;
        }
        return GameState::Playing;
    }

    // SE Vx, Vy
    fn opcode_5(&mut self, opcode: u16) -> GameState {
        if self.registers.v[get_x(opcode)] == self.registers.v[get_y(opcode)] {
            self.registers.pc += 2;
        }
        return  GameState::Playing;
    }

    // LD Vx
    fn opcode_6(&mut self, opcode: u16) -> GameState {
        self.registers.v[get_x(opcode)] = get_kk(opcode);
        return  GameState::Playing;
    }

    // ADD Vx
    fn opcode_7(&mut self, opcode: u16) -> GameState {
        let x = get_x(opcode) as u8;
        let value = self.registers.v[x as usize];
        self.registers.v[x as usize] = value.wrapping_add(get_kk(opcode));
        return GameState::Playing;
    }

    // Bitwise, Carry Arithmetic Instructions
    fn opcode_8(&mut self, opcode: u16) -> GameState {
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
    
        return GameState::Playing;
    }

    fn opcode_9(&mut self, opcode: u16) -> GameState {
        if self.registers.v[get_x(opcode)] != self.registers.v[get_y(opcode)] {
            self.registers.pc += 2;
        }
        return  GameState::Playing;
    }

    // LD I
    fn opcode_a(&mut self, opcode: u16) -> GameState {
        self.registers.i = opcode & get_nnn(opcode);
        return  GameState::Playing;
    }

    // JP V0, nnn
    fn opcode_b(&mut self, opcode: u16) -> GameState {
        self.registers.pc = self.registers.v[0] as u16 + get_nnn(opcode);
        return  GameState::Playing;
    }

    // RND Vx, kk
    fn opcode_c(&mut self, opcode: u16) -> GameState {
        let rnd = rand::random::<u8>();
        self.registers.v[get_x(opcode)] = rnd & get_kk(opcode);
        return  GameState::Playing;
    }

    // DRW
    fn opcode_d(&mut self, opcode: u16) -> GameState {
        let x = self.registers.v[get_x(opcode)] % DISPLAY_WIDTH as u8;
        let y = self.registers.v[get_y(opcode)] % DISPLAY_HEIGHT as u8;
        let i = self.registers.i;

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

        return  GameState::Playing;
    }

    fn opcode_e(&mut self, opcode: u16) -> GameState {
        let x = self.registers.v[get_x(opcode)];
        match get_kk(opcode) {
            // SKP Vx
            0x9e => {
                if is_pressed(x, &self.keyboard) {
                    self.registers.pc += 2;
                }
            },
            // SKNP Vx
            0xa1 => {
                if !is_pressed(x, &self.keyboard) {
                    self.registers.pc += 2;
                }
            },
            _ => println!("Invalid opcode {:#X}", opcode),
        }

        return  GameState::Playing;
    }

    fn opcode_f(&mut self, opcode: u16) -> GameState {
        match get_kk(opcode) {
            // LD Vx, DT
            0x07 => {
                self.registers.v[get_x(opcode)] = self.registers.dt;
                return GameState::Playing;
            },
            // LD Vx, K
            0x0a => {
                return GameState::Paused;
            },
            // LD DT, Vx
            0x15 => {
                self.registers.dt = self.registers.v[get_x(opcode)];
                return GameState::Playing;
            },
            // LD ST, Vx
            0x18 => {
                self.registers.st = self.registers.v[get_x(opcode)];
                return GameState::Playing;
            },
            // ADD I, Vx
            0x1e => {
                self.registers.i += self.registers.v[get_x(opcode)] as u16;
                return GameState::Playing;
            },
            // LD F, Vx
            0x29 => {
                let x = self.registers.v[get_x(opcode)] & 0xf;
                self.registers.i = x as u16 
                    * FONT_HEIGHT as u16
                    + FONT_RAM_START as u16;
                return GameState::Playing;
            },
            // LD BCD, Vx
            0x33 => {
                let x = get_x(opcode) as u8;
                self.bus.write_byte(self.registers.i as usize, x / 100);
                self.bus.write_byte((self.registers.i + 1) as usize, x % 100 / 10);
                self.bus.write_byte((self.registers.i + 2) as usize, x % 10);
                return GameState::Playing;
            },
            // LD [I], Vx
            0x55 => {
                let i = self.registers.i as usize;
                for j in 0..=get_x(opcode) as usize {
                    self.bus.write_byte(i + j, self.registers.v[j]);
                }
                return GameState::Playing;
            },
            // LD Vx, [I]
            0x65 => {
                let i = self.registers.i;
                for j in 0..=get_x(opcode) {
                    self.registers.v[j] = self.bus.read_byte(i + j as u16);
                }
                return GameState::Playing;
            },
            _ => {
                println!("Invalid opcode {:#X}", opcode);
                return GameState::Playing;
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