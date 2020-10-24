extern crate sdlgame;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
// use sdl2::keyboard::Mod;

use sdlgame::{KeyState, KeyStates, KeyboardState};

struct State {
    bgcolor: Color,
}

fn main() -> Result<(), String> {
    let context = sdl2::init().expect("sdl2::init failed");
    let video_subsystem = context.video().expect("video subsytem init failed");

    let window = video_subsystem
        .window("Keyboard Events", 800, 600)
        .position_centered()
        .build()
        .expect("unable to create window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("unable to create canvas");

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
        kbstate.update_state(&event_pump);

        state.handle_r(kbstate.get_keystate(Scancode::R));
        state.handle_g(kbstate.get_keystate(Scancode::G));
        state.handle_b(kbstate.get_keystate(Scancode::B));

        canvas.set_draw_color(state.bgcolor);
        canvas.clear();
        canvas.present();
    }
    Ok(())
}

impl State {
    fn handle_r(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            if key.left_shift == KeyStates::Down || key.right_shift == KeyStates::Down {
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
    fn handle_g(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            if key.left_shift == KeyStates::Down || key.right_shift == KeyStates::Down {
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
    fn handle_b(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            if key.left_shift == KeyStates::Down || key.right_shift == KeyStates::Down {
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
