extern crate sdl3;

use sdl3::Sdl;
use sdl3::render::Canvas;
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::rect::Rect;
use sdl3::video::Window;
use std::time::Duration;
use std::thread;

const BASE_W: u32 = 64;
const BASE_H: u32 = 32;
const SCALE_FACTOR: u32 = 10;

struct Graphics {
    sdl_context: Sdl,
    canvas: Canvas<Window>
}
impl Graphics {
    pub fn new () -> Graphics {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(
            "CHIP-9 🍐",
            BASE_W * SCALE_FACTOR,
            BASE_H * SCALE_FACTOR
        )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        return Graphics {
            sdl_context: sdl_context,
            canvas: canvas
        }
    }
}

struct Screen {
    pixels: [[bool;64];32]
    // draw returns an array of rects
    // set_pixel method takes row, column
}

pub fn draw() {
    let graphics = Graphics::new();
    let mut canvas = graphics.canvas;
    let mut event_pump = graphics.sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        let pixel: Rect = Rect::new(
            (0 * SCALE_FACTOR).try_into().unwrap(),
            (0 * SCALE_FACTOR).try_into().unwrap(),
            1 * SCALE_FACTOR,
            1 * SCALE_FACTOR,
        );
        canvas.fill_rect(pixel).unwrap();

        let pixels: [[bool;64];32] = [[false;64];32];
        let screen = Screen{ pixels };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// Take Hexidecimal instructions
// Turn into an array of rects to fill
// fixed-size array of 64x32 of booleans
// fill_rect can take array of rects to fill