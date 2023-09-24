use std::{
    thread,
    time::{Duration, Instant}
};

use constants::{MS_PER_FRAME, MICROS_PER_CYCLE};

use crate::cpu::CPU;

mod audio;
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

        state = match cpu.handle_input(&state) {
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
        } else if state == GameState::Paused {
            cpu.check_input(&mut state);
        }

        if fps_timer.elapsed().as_millis() as f32 >= MS_PER_FRAME {
            if state == GameState::Playing { cpu.render(); }

            cpu.decrement_timers();

            fps_timer = Instant::now();
        }

        cpu.play_audio();

        let remaining = MICROS_PER_CYCLE - cycle_timer.elapsed().as_micros() as f32;
        thread::sleep(Duration::from_micros(remaining as u64));
    }

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    Playing,
    Paused,
    Ended,
}