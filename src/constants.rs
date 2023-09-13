pub const RAM_SIZE: u16 = 0x0fff;

pub const REGISTER_COUNT: u8 = 8;
pub const STACK_SIZE: u8 = 8;

pub const WINDOW_TITLE: &str = "CHIP-8r";
pub const DISPLAY_WIDTH: u32 = 64;
pub const DISPLAY_HEIGHT: u32 = 32;
pub const PIXEL_SIZE: u32 = 15;
pub const WINDOW_WIDTH: u32 = DISPLAY_WIDTH * PIXEL_SIZE;
pub const WINDOW_HEIGHT: u32 = DISPLAY_HEIGHT * PIXEL_SIZE;

pub const KEYBOARD_SIZE: usize = 0xf;
