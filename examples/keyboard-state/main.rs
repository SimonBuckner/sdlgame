extern crate sdlgame;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;

use sdlgame::keyboard::{KeyboardState};

const TITLE: &str = "Keyboard State Demo - R,G,B increase Red,Green,Blue. Shift+R/G/B descreases";
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;

struct State {
    bgcolor: Color,
}

fn main() -> Result<(), String> {
    let ctx_can = sdlgame::standard_800_600_canvas(TITLE, MAX_WIDTH, MAX_HEIGHT);
    let context = ctx_can.0;
    let mut canvas = ctx_can.1;

    let mut state = State {
        bgcolor: Color::RGB(0, 0, 0),
    };

    let mut event_pump = context.event_pump()?;

    let mut kbstate = KeyboardState::new(&event_pump);

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
        kbstate.update(&event_pump);

        state.handle_r(&kbstate);
        state.handle_g(&kbstate);
        state.handle_b(&kbstate);

        let title = format!("{} - R({}) G({}) B({})", TITLE, state.bgcolor.r, state.bgcolor.g,state.bgcolor.b);
        canvas = sdlgame::set_window_title(canvas, &title);
        
        canvas.set_draw_color(state.bgcolor);
        canvas.clear();
        canvas.present();
    }
    Ok(())
}

impl State {
    fn handle_r(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::R) {
            if keyb.is_down(Scancode::LShift) || keyb.is_down(Scancode::RShift) {
                if self.bgcolor.r != 0 {
                    self.bgcolor.r -= 1;
                }
            } else {
                if self.bgcolor.r < 255 {
                    self.bgcolor.r += 1;
                }
            }
        }
    }
    fn handle_g(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::G) {
            if keyb.is_down(Scancode::LShift) || keyb.is_down(Scancode::RShift) {
                if self.bgcolor.g != 0 {
                    self.bgcolor.g -= 1;
                }
            } else {
                if self.bgcolor.g < 255 {
                    self.bgcolor.g += 1;
                }
            }
        }
    }
    fn handle_b(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::B) {
            if keyb.is_down(Scancode::LShift) || keyb.is_down(Scancode::RShift) {
                if self.bgcolor.b != 0 {
                    self.bgcolor.b -= 1;
                }
            } else {
                if self.bgcolor.b < 255 {
                    self.bgcolor.b += 1;
                }
            }
        }
    }
}
