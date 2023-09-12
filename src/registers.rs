const REGISTER_COUNT: u8 = 8;
const STACK_SIZE: u8 = 8;
pub struct Registers {
    // Program Counter
    pub pc: u16,
    // Address Register
    pub i: u16,
    // Stack Pointer
    pub sp: u8,
    pub stack: [u16; STACK_SIZE as usize],
    // Variable Registers
    pub v: [u8; REGISTER_COUNT as usize],
    // Delay Timer
    pub dt: u8,
    // Sound Timer
    pub st: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0x200,
            i: 0,
            sp: 0,
            stack: [0; STACK_SIZE as usize],
            v: [0; REGISTER_COUNT as usize],
            dt: 0,
            st: 0
        }
    }
}
