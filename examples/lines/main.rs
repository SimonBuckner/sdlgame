extern crate sdlgame;

use rand::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use sdlgame::{KeyState, KeyStates, KeyboardState};

const INITIAL_LINES: usize = 50;
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
    color: Color,
}

#[derive(Debug)]
struct State {
    bgcolor: Color,
    lines: Vec<Line>,
}

fn main() -> Result<(), String> {
    let context = sdl2::init().expect("sdl2::init failed");
    let video_subsystem = context.video().expect("video subsytem init failed");

    let window = video_subsystem
        .window(
            "Lines Demo. +/- to change line numbers / F5 to randminse lines / F6 to randomise colours",
            MAX_WIDTH,
            MAX_HEIGHT,
        )
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
        lines: Vec::new(),
    };

    state.create_random_lines(INITIAL_LINES);

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
        state.handle_f5(kbstate.get_keystate(Scancode::F5));
        state.handle_f6(kbstate.get_keystate(Scancode::F6));
        state.handle_kp_plus(kbstate.get_keystate(Scancode::KpPlus));
        state.handle_kp_minus(kbstate.get_keystate(Scancode::KpMinus));
        state.handle_plus(kbstate.get_keystate(Scancode::Equals));
        state.handle_minus(kbstate.get_keystate(Scancode::Minus));

        canvas.set_draw_color(state.bgcolor);
        canvas.clear();

        state.draw_lines(&mut canvas);

        canvas.present();
    }
    Ok(())
}

impl State {
    fn create_random_lines(&mut self, lines: usize) {
        
        self.lines.clear();
        for _i in 0..lines {
            
            self.lines.push(new_random_line());
        }
    }
    
    fn randomise_line_colors(&mut self) {
        let mut rng = thread_rng();

        for line in &mut self.lines {

            let c = Color::RGB(
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
            );
            line.color = c;
        }
    }

    fn add_new_line(&mut self) {
        self.lines.push(new_random_line())
    }

    fn remove_oldest_line(&mut self) {
        if self.lines.len() > 0 {
            self.lines.drain(0..1);
        }
    }

    fn draw_lines(&self, canvas: &mut WindowCanvas) {
        for line in self.lines.iter() {
            canvas.set_draw_color(line.color);
            let _= canvas.draw_line(line.p1, line.p2);
        }
    }

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

    fn handle_f5(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.create_random_lines(self.lines.len());
        }
    }
    fn handle_f6(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.randomise_line_colors();
        }
    }

    fn handle_kp_plus(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.add_new_line();
        }
    }
    fn handle_kp_minus(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.remove_oldest_line();
        }
    }


    fn handle_plus(&mut self, key: KeyState) {
        if key.state == KeyStates::Down && key.shift == KeyStates::Down {
            self.add_new_line();
        }
    }
    fn handle_minus(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.remove_oldest_line();
        }
    }
}

fn new_random_line() -> Line {
    let mut rng = thread_rng();

    let c = Color::RGB(
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
    );

    let line = Line {
        p1: Point::new(
            rng.gen_range(0, MAX_WIDTH + 1) as i32,
            rng.gen_range(0, MAX_HEIGHT + 1) as i32,
        ),
        p2: Point::new(
            rng.gen_range(0, MAX_WIDTH + 1) as i32,
            rng.gen_range(0, MAX_HEIGHT + 1) as i32,
        ),
        color: c,
    };
    line
}