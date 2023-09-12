use crate::cpu::CPU;

mod bus;
mod cpu;
mod error;
mod registers;

fn main() {
    let mut cpu = CPU::new();
    // let keyboard = Keyboard::new();
    // let display = Display::new();
    // let audio = Audio::new();

    let mut running = true;
    while running {
        // keyboard.handle_input();
        match cpu.cycle() {
            Ok(()) => (),
            Err(error) => {
                dbg!(error);
                running = false;
            },

        }
        // display.render();
        // audio.emit();
        // cpu.adjust_cycles();
    }
}
