use std::{
    error::Error,
    fs::File,
    io::Read,
};

use crate::constants::{
    FONT_RAM_START,
    FONT_RAM_END,
    PROGRAM_RAM_START,
    PROGRAM_RAM_END, RAM_SIZE,
    FONT_HEIGHT
};

type RamType = [u8; (RAM_SIZE - 1) as usize];
pub struct Bus {
    ram: RamType
}

impl Bus {
    pub fn new() -> Bus {
        let mut ram = [0 as u8; (RAM_SIZE - 1) as usize];
        load_fonts(&mut ram);

        // load_test_rom(&mut ram);

        match load_rom(&mut ram) {
            Ok(()) => println!("ROM load successful"),
            Err(x) => println!("Error: {}", x),
        }

        return Bus { ram };
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        return self.ram[(addr & 0xffff) as usize];
    }

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

fn load_rom(ram: &mut RamType) -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u8> = Vec::new();
    // let mut file = File::open("./roms/1-chip8-logo.ch8")?;
    // let mut file = File::open("./roms/2-ibm-logo.ch8")?;
    // let mut file = File::open("./roms/3-corax+.ch8")?;
    let mut file = File::open("./roms/4-flags.ch8")?;
    // let mut file = File::open("./roms/5-quirks.ch8")?;
    // let mut file = File::open("./roms/6-keypad.ch8")?;
    let rom_size = file.read_to_end(&mut buffer)?;

    if rom_size > PROGRAM_RAM_END - PROGRAM_RAM_START {
        return Err("invalid ROM".into());
    }

    for i in 0..buffer.len() {
        ram[PROGRAM_RAM_START + i] = buffer[i];
    }

    Ok(())
}

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