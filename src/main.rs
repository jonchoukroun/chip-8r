use std::{
    thread,
    time::{Duration, Instant}
};

use constants::{MS_PER_FRAME, MS_PER_CYCLE};

use crate::cpu::CPU;

mod bus;
mod constants;
mod cpu;
mod display;
mod error;
mod keyboard;
mod registers;

fn main() -> Result<(), String> {
    let mut cpu = CPU::new()?;

    let mut state = GameState::Playing;
    let mut fps_timer = Instant::now();
    while state != GameState::Ended {
        let cycle_timer = Instant::now();

        state = match cpu.handle_input() {
            GameState::Ended => { break; },
            _ if state == GameState::Paused => GameState::Paused,
            new_state => new_state,
        };

        if state == GameState::Playing {
            state = match cpu.cycle() {
                Ok(new_state) => new_state,
                Err(error) => {
                    dbg!(error);
                    break;
                },

            };
        }

        if fps_timer.elapsed().as_millis() as f32 >= MS_PER_FRAME {
            if state == GameState::Playing {
                cpu.render();
            }

            // TODO: sound timer

            // TODO: delay timer

            fps_timer = Instant::now();
        }

        // TODO: audio.emit();

        let remaining = MS_PER_CYCLE - cycle_timer.elapsed().as_millis() as f32;
        thread::sleep(Duration::from_millis(remaining as u64));
    }

    Ok(())
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    Playing,
    Paused,
    Ended,
}