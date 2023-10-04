extern crate sdl2;

use std::{
    error::Error,
    fs::File,
    io::Read,
};

use crate::audio::Audio;
use crate::constants::{
    FONT_RAM_START,
    FONT_RAM_END,
    PROGRAM_RAM_START,
    PROGRAM_RAM_END, RAM_SIZE,
    FONT_HEIGHT
};
use crate::cpu::FrameBuffer;
use crate::display::Display;
use crate::keyboard::Keyboard;

type RamType = [u8; (RAM_SIZE - 1) as usize];
pub struct Bus {
    audio: Audio,
    display: Display,
    pub keyboard: Keyboard,
    ram: RamType
}

impl Bus {
    pub fn new() -> Result<Bus, String> {
        let sdl_context = sdl2::init()?;

        let audio = Audio::new(&sdl_context)?;
        let display = Display::new(&sdl_context)?;
        let keyboard = Keyboard::new(&sdl_context)?;

        Ok(Bus {
            audio,
            display,
            keyboard,
            ram: [0; (RAM_SIZE - 1) as usize],
        })
    }

    pub fn init_ram(&mut self) {
        load_fonts(&mut self.ram);
    }

    pub fn load_rom(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buffer: Vec<u8> = Vec::new();
        // let mut file = File::open("./roms/1-chip8-logo.ch8")?;
        let mut file = File::open("./roms/2-ibm-logo.ch8")?;
        // let mut file = File::open("./roms/3-corax+.ch8")?;
        // let mut file = File::open("./roms/4-flags.ch8")?;
        // let mut file = File::open("./roms/5-quirks.ch8")?;
        // let mut file = File::open("./roms/6-keypad.ch8")?;
        let rom_size = file.read_to_end(&mut buffer)?;

        if rom_size > PROGRAM_RAM_END - PROGRAM_RAM_START {
            return Err("invalid ROM".into());
        }

        self.ram[PROGRAM_RAM_START..(buffer.len() + PROGRAM_RAM_START)]
            .copy_from_slice(&buffer[..]);

        Ok(())
    }

    pub fn handle_input(&mut self) -> bool { self.keyboard.handle_input() }

    pub fn render(&mut self, buffer: &FrameBuffer) {
        self.display.render(buffer);
    }

    pub fn play_audio(&mut self) {
        if !self.audio.is_playing() {
            self.audio.play();
        }
    }

    pub fn stop_audio(&mut self) {
        if self.audio.is_playing() {
            self.audio.stop();
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool { self.keyboard.is_pressed(key) }

    pub fn get_keyup(&self) -> Option<u8> { self.keyboard.get_keyup() }

    pub fn read_byte(&self, addr: u16) -> u8 { self.ram[addr as usize] }

    pub fn write_byte(&mut self, addr: usize, byte: u8) {
        self.ram[addr] = byte;
    }
}

fn load_fonts(ram: &mut RamType) {
    for i in FONT_RAM_START..FONT_RAM_END {
        ram[i] = FONT_SPRITES[i / FONT_HEIGHT][i % FONT_HEIGHT];
    }
}

// fn load_test_rom(ram: &mut RamType) {
//     let rom: [u8; 6] = [
//         // LD v0, 3 sec = 0xb4
//         0x60, 0xb4,
//         // LD ST, V0
//         0xf0, 0x18,
//         // LD v1, K
//         0xf1, 0x0a
//     ];
//     for i in 0..rom.len() {
//         ram[PROGRAM_RAM_START + i] = rom[i];
//     }
// }

type FontHex = [u8; FONT_HEIGHT];
const FONT_SPRITES: [FontHex; 16] = [
    // 0, 0
    [0xf0, 0x90, 0x90, 0x90, 0xf0],
    // 1, 5
    [0x20, 0x60, 0x20, 0x20, 0x70],
    // 2, a
    [0xf0, 0x10, 0xf0, 0x80, 0xf0],
    // 3, f
    [0xf0, 0x10, 0xf0, 0x10, 0xf0],
    // 4, 14
    [0x90, 0x90, 0xf0, 0x10, 0x10],
    // 5
    [0xf0, 0x80, 0xf0, 0x10, 0xf0],
    // 6
    [0xf0, 0x80, 0xf0, 0x90, 0xf0],
    // 7
    [0xf0, 0x10, 0x20, 0x40, 0x40],
    // 8
    [0xf0, 0x90, 0xf0, 0x90, 0xf0],
    // 9
    [0xf0, 0x90, 0xf0, 0x10, 0xf0],
    // a
    [0xf0, 0x90, 0xf0, 0x90, 0x90],
    // b
    [0xe0, 0x90, 0xe0, 0x90, 0xe0],
    // c
    [0xf0, 0x80, 0x80, 0x80, 0xf0],
    // d
    [0xe0, 0x90, 0x90, 0x90, 0xe0],
    // e
    [0xf0, 0x80, 0xf0, 0x80, 0xf0],
    // f
    [0xf0, 0x80, 0xf0, 0x80, 0x80]
];