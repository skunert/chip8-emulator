const SCALING_FACTOR: u32 = 8;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Display {
    canvas: WindowCanvas,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Display {
        let canvas = Display::setup_display(sdl_context).expect("Unable to set up canvas");
        Display { canvas: canvas }
    }

    fn setup_display(sdl_context: &sdl2::Sdl) -> Result<WindowCanvas, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(
                "rust-sdl2 demo: Video",
                64 * SCALING_FACTOR,
                32 * SCALING_FACTOR,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();
        Ok(canvas)
    }

    pub fn draw(&mut self, pixels: [[u8; 64]; 32]) {
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas
            .fill_rect(None)
            .expect("Unable to fill rectangle on screen.");
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (row_index, vec) in pixels.iter().enumerate() {
            for (col_index, value) in vec.iter().enumerate() {
                if *value == 1 {
                    self.canvas
                        .fill_rect(Rect::new(
                            col_index as i32 * SCALING_FACTOR as i32,
                            row_index as i32 * SCALING_FACTOR as i32,
                            SCALING_FACTOR,
                            SCALING_FACTOR,
                        ))
                        .expect("Unable to draw");
                }
            }
        }
        self.canvas.present();
    }
}
