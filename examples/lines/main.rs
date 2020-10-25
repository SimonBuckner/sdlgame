extern crate sdlgame;

use rand::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use sdlgame::keyboard::KeyboardState;

const TITLE: &str = "Lines Demo - R,G,B increase Red,Green,Blue. Shift+R/G/B descreases - +/- Increases/decreases lines";
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;
const INITIAL_LINES: usize = 50;

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
    let ctx_can = sdlgame::standard_800_600_canvas(TITLE, MAX_WIDTH, MAX_HEIGHT);
    let context = ctx_can.0;
    let mut canvas = ctx_can.1;

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
        kbstate.update(&event_pump);

        state.handle_r(&kbstate);
        state.handle_g(&kbstate);
        state.handle_b(&kbstate);
        state.handle_f5(&kbstate);
        state.handle_f6(&kbstate);
        state.handle_kp_plus(&kbstate);
        state.handle_kp_minus(&kbstate);
        state.handle_plus(&kbstate);
        state.handle_minus(&kbstate);

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

    fn handle_f5(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::F5) {
            self.create_random_lines(self.lines.len());
        }
    }
    fn handle_f6(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::F6) {
            self.randomise_line_colors();
        }
    }

    fn handle_kp_plus(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::KpPlus) {
            self.add_new_line();
        }
    }
    fn handle_kp_minus(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::KpMinus) {
            self.remove_oldest_line();
        }
    }

    fn handle_plus(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::Equals) {
            if keyb.is_down(Scancode::LShift) || keyb.is_down(Scancode::RShift) {
                self.add_new_line();
            }
        }
    }
    fn handle_minus(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::Minus) {
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