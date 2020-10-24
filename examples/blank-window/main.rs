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

#[allow(dead_code)]
struct State {
    // Sdl State
    context: Sdl,
    canvas: Canvas<Window>,

    // Game state
    bgcolor: Color,
}

fn main() -> Result<(), String> {
    let context = sdl2::init().expect("sdl2::init failed");
    let video_subsystem = context.video().expect("video subsytem init failed");
    let window = video_subsystem
        .window("Blank Window Demo. Press ALT+F4 to close or click the Esc", 800, 600)
        .position_centered()
        .build()
        .expect("unable to create window");
    let canvas = window
        .into_canvas()
        .build()
        .expect("unable to create canvas");
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
