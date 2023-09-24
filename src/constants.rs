pub const FONT_RAM_START: usize = 0x000;
pub const FONT_RAM_END: usize = FONT_RAM_START + (16 * 5);
pub const FONT_HEIGHT: usize = 5;
pub const PROGRAM_RAM_START: usize = 0x200;
pub const PROGRAM_RAM_END: usize = 0xe8f;
pub const RAM_SIZE: u16 = 0x0fff;

pub const REGISTER_COUNT: u8 = 16;
pub const FLAG_REGISTER: usize = 0xf;
pub const STACK_SIZE: u8 = 8;
pub const BIT_MASK: u8 = 0b10000000;

pub const WINDOW_TITLE: &str = "CHIP-8r";
pub const DISPLAY_WIDTH: u32 = 64;
pub const DISPLAY_HEIGHT: u32 = 32;
pub const PIXEL_SIZE: u32 = 15;
pub const WINDOW_WIDTH: u32 = DISPLAY_WIDTH * PIXEL_SIZE;
pub const WINDOW_HEIGHT: u32 = DISPLAY_HEIGHT * PIXEL_SIZE;
pub const SPRITE_WIDTH: u8 = 8;
pub const BG_RED: u8 = 28;
pub const BG_GREEN: u8 = 28;
pub const BG_BLUE: u8 = 28;
pub const FG_RED: u8 = 51;
pub const FG_GREEN: u8 = 255;
pub const FG_BLUE: u8 = 51;

pub const KEYBOARD_SIZE: usize = 16;

pub const MS_PER_FRAME: f32 = 1000.0 / 60.0;
pub const MICROS_PER_CYCLE: f32 = 1_000_000.0 / 700.0;

pub const SAMPLE_RATE: f32 = 441000.0;
pub const CHANNELS: u8 = 1;
pub const WAVETABLE_SIZE: usize = 128;
pub const PITCH: f32 = 330.0;
pub const VOLUME: f32 = 0.25;