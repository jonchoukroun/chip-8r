extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;

use crate::cpu::FrameBuffer;

const WINDOW_TITLE: &str = "CHIP-8r";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 320;

pub struct Display {
    canvas: Canvas<Window>,
}
impl Display {
    pub fn new(sdl_context: &Sdl) -> Result<Display, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window(
            WINDOW_TITLE,
            WINDOW_WIDTH,
            WINDOW_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
            
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        return Ok(Display { canvas });
    }

    pub fn render(&mut self, buffer: FrameBuffer) {
        self.canvas.set_draw_color(Color::CYAN);
        for y in 0..32 {
            for x in 0..64 {
                if buffer[y * 64 + x] == 1 {
                    self.canvas.fill_rect(Rect::new(
                        (x * 10) as i32,
                        (y * 10) as i32,
                        10,
                        10
                    )).unwrap();
                }
            }
        }
        self.canvas.present();
    }

    fn clear_canvas(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.present();
    }
}