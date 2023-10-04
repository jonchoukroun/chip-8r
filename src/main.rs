mod audio;
mod bus;
mod constants;
mod cpu;
mod display;
mod error;
mod keyboard;
mod registers;

use crate::cpu::Cpu;

fn main() -> Result<(), String> {
    let mut cpu = Cpu::new()?;

    cpu.run();

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    Playing,
    Paused,
    Ended,
}