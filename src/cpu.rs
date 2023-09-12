use crate::bus::Bus;
use crate::error::{Error, ErrorType};
use crate::registers::Registers;

pub struct CPU {
    bus: Bus,
    registers: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            bus: Bus::new(),
            registers: Registers::new(),
        }
    }

    pub fn cycle(&mut self) -> Result<(), Error> {
        let opcode = match self.fetch() {
            Ok(instruction) => instruction,
            Err(e) => return Err(e),
        };

        match opcode {
            0x0000 => println!("0x000"),
            _ => println!("Other"),
        }

        Ok(())
    }

    fn fetch(&mut self) -> Result<u16, Error> {
        if self.registers.pc < 0x200 || self.registers.pc >= 0xffe {
            return Err(Error::new(ErrorType::InaccessibleMemoryAddress))
        }
        let high = self.bus.read_byte(self.registers.pc) as u16;
        self.registers.pc += 1;
        let low = self.bus.read_byte(self.registers.pc) as u16;
        self.registers.pc += 1;

        println!(
            "CPU fetch | opcode = {}, PC = {}",
            (high << 0x8 & low), self.registers.pc);

        return Ok((high << 0x8) & low);
    }
}