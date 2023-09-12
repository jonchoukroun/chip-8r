const RAM_SIZE: u16 = 0x0fff;
pub struct Bus {
    ram: [u8; (RAM_SIZE - 1) as usize],
}

impl Bus {
    pub fn new() -> Bus {
        let ram = [0 as u8; (RAM_SIZE - 1) as usize];
        // TODO: write fonts in memory
        return Bus { ram };
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        return self.ram[(addr & 0x0fff) as usize];
    }

    pub fn write_byte(&mut self, addr: u8, byte: u8) {
        return self.ram[addr as usize] = byte;
    }
}
