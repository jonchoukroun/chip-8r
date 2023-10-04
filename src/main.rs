mod audio;
mod bus;
mod constants;
mod cpu;
mod display;
mod error;
mod keyboard;
mod registers;

extern crate native_dialog;

use native_dialog::{MessageDialog, MessageType};

use crate::cpu::Cpu;

fn main() -> Result<(), String> {
    let mut cpu = match Cpu::new() {
        Ok(cpu) => cpu,
        Err(_) => {
            handle_fatal_error();
            return Ok(());
        }
    };

    if !cpu.run() {
        handle_fatal_error();
    }

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    Playing,
    Paused,
    Ended,
}

fn handle_fatal_error() {
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title("Chip-8 Crashed!")
        .set_text("Something went wrong and Chip-8 will quit, sorry.")
        .show_alert()
        .unwrap();
}