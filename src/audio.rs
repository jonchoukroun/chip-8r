use sdl2::{Sdl, audio::{AudioDevice, AudioCallback, AudioSpecDesired}};

use crate::constants::{WAVETABLE_SIZE, PITCH, SAMPLE_RATE, VOLUME, CHANNELS};

pub struct Audio {
    // oscillator: Oscillator,
    device: AudioDevice<Oscillator>,
    playing: bool,
}

impl Audio {
    pub fn new(sdl_context: &Sdl) -> Result<Audio, String> {
        let audio_subsystem = sdl_context.audio()?;
        
        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLE_RATE as i32),
            channels: Some(CHANNELS),
            samples: None,
        };

        let device = audio_subsystem.open_playback(
            None,
            &desired_spec,
            |spec| {
                println!("spec: {:?}", spec);
                return Oscillator::new();
        })?;

        return Ok(Audio {
            device,
            playing: false,
        });
    }

    pub fn play(&mut self) {
        self.device.resume();
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.device.pause();
        self.playing = false;
    }

    // Getters
    pub fn is_playing(&self) -> bool { self.playing }
}

type WaveTable = [f32; WAVETABLE_SIZE];
struct Oscillator {
    table: WaveTable,
    cursor: f32,
    phase_inc: f32,
}

impl Oscillator {
    pub fn new() -> Oscillator {
        let mut table: WaveTable = [0.0; WAVETABLE_SIZE];
        for i in 0..table.len() {
            table[i] = (
                2.0 * (i as  f32 / WAVETABLE_SIZE as f32).tan().atan()
            ).sin();
        }

        let phase_inc = PITCH * WAVETABLE_SIZE as f32 / SAMPLE_RATE;

        return Oscillator {
            table,
            cursor: 0.0,
            phase_inc,
        };
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.cursor += self.phase_inc;
        self.cursor %= WAVETABLE_SIZE as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let first_index = self.cursor as usize;
        let next_index = (first_index + 1) % WAVETABLE_SIZE;

        let next_index_weight = self.cursor - first_index as f32;
        let first_index_weight = 1.0 - next_index_weight;

        return first_index_weight * self.table[first_index]
            + next_index_weight * self.table[next_index];
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

impl AudioCallback for Oscillator {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.get_sample() * VOLUME;
        }
    }
}