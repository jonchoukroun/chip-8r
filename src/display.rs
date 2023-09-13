extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;

use crate::cpu::FrameBuffer;
use crate::constants::{
    DISPLAY_WIDTH,
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    WINDOW_TITLE,
    PIXEL_SIZE
};

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
            
        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        return Ok(Display { canvas });
    }

    pub fn render(&mut self, buffer: FrameBuffer) {
        self.clear_canvas();

        self.canvas.set_draw_color(Color::CYAN);
        for i in 0..buffer.len() {
            if buffer[i] == 1 {
                let x = (i as i32 % DISPLAY_WIDTH as i32)
                    * PIXEL_SIZE as i32;
                let y = (i as i32 / DISPLAY_WIDTH as i32)
                    * PIXEL_SIZE as i32;
                let rect = Rect::new(
                    x,
                    y,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                );
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.canvas.present();
    }

    fn clear_canvas(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }
}