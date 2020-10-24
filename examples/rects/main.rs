extern crate sdlgame;

use rand::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use sdlgame::{KeyState, KeyStates, KeyboardState};

const INITIAL_RECTS: usize = 50;
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;
const MAX_RECT_WIDTH: u32 = 100;
const MAX_RECT_HEIGHT: u32 = 100;

#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
    filled: bool,
}

#[derive(Debug)]
struct State {
    bgcolor: Color,
    rects: Vec<Rect>,
    fill_mode: FillMode,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum FillMode {
    None,
    Some,
    All,
}

fn main() -> Result<(), String> {
    let context = sdl2::init().expect("sdl2::init failed");
    let video_subsystem = context.video().expect("video subsytem init failed");

    let window = video_subsystem
        .window(
            "Rects Demo. +/- to change line numbers / F5 to randminse lines / F6 to randomise colour / F to randomise Fills",
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
        rects: Vec::new(),
        fill_mode: FillMode::None,
    };

    state.create_random_rects(INITIAL_RECTS);

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
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    repeat: false,
                    ..
                } => {
                    state.handle_f();
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

        state.draw_rects(&mut canvas);

        canvas.present();
    }
    Ok(())
}

impl State {
    fn create_random_rects(&mut self, rects: usize) {
        self.rects.clear();
        for _i in 0..rects {
            self.rects.push(new_random_rect(self.fill_mode));
        }
    }
    fn randomise_rect_colors(&mut self) {
        let mut rng = thread_rng();

        for rect in &mut self.rects {
            let c = Color::RGB(
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
            );
            rect.color = c;
        }
    }

    fn fill_no_rects(&mut self) {
        for rect in &mut self.rects {
            rect.filled = false;
        }
    }

    fn fill_some_rects(&mut self) {
        let mut rng = thread_rng();

        for rect in &mut self.rects {
            rect.filled = rng.gen_range(0,2) == 0;
        }
    }

    fn fill_all_rects(&mut self) {
        for rect in &mut self.rects {
            rect.filled = true;
        }
    }

    fn add_new_rect(&mut self) {
        self.rects.push(new_random_rect(self.fill_mode))
    }

    fn remove_oldest_rect(&mut self) {
        if self.rects.len() > 0 {
            self.rects.drain(0..1);
        }
    }

    fn cycle_fill_mode(&mut self) {
        match self.fill_mode {
            FillMode::None => {
                self.fill_mode = FillMode::Some;
                self.fill_some_rects();
            }
            FillMode::Some => {
                self.fill_mode = FillMode::All;
                self.fill_all_rects();
            }
            FillMode::All => {
                self.fill_mode = FillMode::None;
                self.fill_no_rects();
            }
        }
    }

    fn draw_rects(&self, canvas: &mut WindowCanvas) {
        for rect in self.rects.iter() {
            canvas.set_draw_color(rect.color);
            let r = sdl2::rect::Rect::new(rect.x, rect.y, rect.width as u32, rect.height as u32);
            if rect.filled {
                let _ = canvas.fill_rect(r);
            } else {
                let _ = canvas.draw_rect(r);
            }
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
            self.create_random_rects(self.rects.len());
        }
    }
    fn handle_f6(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.randomise_rect_colors();
        }
    }

    fn handle_kp_plus(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.add_new_rect();
        }
    }
    fn handle_kp_minus(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.remove_oldest_rect();
        }
    }

    fn handle_plus(&mut self, key: KeyState) {
        if key.state == KeyStates::Down && key.shift == KeyStates::Down {
            self.add_new_rect();
        }
    }
    fn handle_minus(&mut self, key: KeyState) {
        if key.state == KeyStates::Down {
            self.remove_oldest_rect();
        }
    }
    fn handle_f(&mut self) {
            self.cycle_fill_mode();
    }
}

fn new_random_rect(fill_mode: FillMode) -> Rect {
    let mut rng = thread_rng();

    let c = Color::RGB(
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
    );

    let filled = match fill_mode {
        FillMode::None => {
             false
        }
        FillMode::Some => {
            rng.gen_range(0, 2) == 0
        }
        FillMode::All => {
            true
        }
    };

    let mut rect = Rect {
        x: rng.gen_range(0, MAX_WIDTH + 1) as i32,
        y: rng.gen_range(0, MAX_HEIGHT + 1) as i32,
        width: rng.gen_range(0, MAX_RECT_WIDTH) as i32,
        height: rng.gen_range(0, MAX_RECT_HEIGHT) as i32,
        color: c,
        filled: filled,
    };

    while rect.x + rect.width >= MAX_WIDTH as i32 {
        rect.x = rng.gen_range(0, MAX_RECT_WIDTH) as i32;
    }

    while rect.y + rect.height >= MAX_HEIGHT as i32 {
        rect.y = rng.gen_range(0, MAX_HEIGHT + 1) as i32;
    }

    rect
}
