extern crate sdlgame;

// use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
// use sdl2::VideoSubsystem;

// use sdlgame::*;

const TITLE: &str = "Blank Window Demo";
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;

#[allow(dead_code)]
struct State {
    // Sdl State
    context: Sdl,
    canvas: Canvas<Window>,

    // Game state
    bgcolor: Color,
}

fn main() -> Result<(), String> {
    let ctx_can = sdlgame::standard_800_600_canvas(TITLE, MAX_WIDTH, MAX_HEIGHT);
    let context = ctx_can.0;
    let canvas = ctx_can.1;

    let mut state = State {
        context: context,
        canvas: canvas,
        bgcolor: Color::RGB(0, 0, 0),
    };

    state.main_loop()?;

    Ok(())
}

impl State {
    fn main_loop(&mut self) -> Result<(), String> {
        let mut event_pump = self.context.event_pump()?;
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'main;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'main;
                    }
                    _ => {}
                }
            }
            self.update_state();

            self.draw();
            self.canvas.present();
        }
        Ok(())
    }

    fn update_state(&mut self) {
        if self.bgcolor.r < 255 {
            self.bgcolor.r += 1;
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(self.bgcolor);
        self.canvas.clear();
    }
}
