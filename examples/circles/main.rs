extern crate sdlgame;

use rand::prelude::*;

use sdl2::gfx::primitives::DrawRenderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use sdlgame::keyboard::KeyboardState;

const INITIAL_CIRCLES: usize = 50;
const MAX_CIRCLE_RADIUS: u32 = 100;
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;

#[derive(Debug)]
struct Circle {
    x: i16,
    y: i16,
    radius: i16,
    color: Color,
    filled: bool,
}

#[derive(Debug)]
struct State {
    bgcolor: Color,
    circles: Vec<Circle>,
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
            "Circles Demo. +/- to change line numbers / F5 to randminse lines / F6 to randomise colours",
            MAX_WIDTH,
            MAX_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .expect("unable to create window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("unable to create canvas");

    let mut state = State {
        bgcolor: Color::RGB(0, 0, 0),
        circles: Vec::new(),
        fill_mode: FillMode::None,
    };

    state.create_random_circles(INITIAL_CIRCLES);

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

        state.draw_circles(&mut canvas);

        canvas.present();
    }
    Ok(())
}

impl State {
    fn create_random_circles(&mut self, circles: usize) {
        self.circles.clear();
        for _i in 0..circles {
            self.circles.push(new_random_circle(self.fill_mode));
        }
    }
    fn randomise_circle_colors(&mut self) {
        let mut rng = thread_rng();

        for circle in &mut self.circles {
            let c = Color::RGB(
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
            );
            circle.color = c;
        }
    }

    fn fill_no_circles(&mut self) {
        for circle in &mut self.circles {
            circle.filled = false;
        }
    }

    fn fill_some_circles(&mut self) {
        let mut rng = thread_rng();

        for circle in &mut self.circles {
            circle.filled = rng.gen_range(0, 2) == 0;
        }
    }

    fn fill_all_circles(&mut self) {
        for circle in &mut self.circles {
            circle.filled = true;
        }
    }

    fn add_new_circle(&mut self) {
        self.circles.push(new_random_circle(self.fill_mode))
    }

    fn remove_oldest_circle(&mut self) {
        if self.circles.len() > 0 {
            self.circles.drain(0..1);
        }
    }

    fn cycle_fill_mode(&mut self) {
        match self.fill_mode {
            FillMode::None => {
                self.fill_mode = FillMode::Some;
                self.fill_some_circles();
            }
            FillMode::Some => {
                self.fill_mode = FillMode::All;
                self.fill_all_circles();
            }
            FillMode::All => {
                self.fill_mode = FillMode::None;
                self.fill_no_circles();
            }
        }
    }

    fn draw_circles(&self, canvas: &mut WindowCanvas) {
        for c in self.circles.iter() {
            if c.filled {
                let _ = canvas.filled_circle(c.x as i16, c.y as i16, c.radius as i16, c.color);
            } else {
                let _ = canvas.circle(c.x as i16, c.y as i16, c.radius as i16, c.color);
            }
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
            self.create_random_circles(self.circles.len());
        }
    }
    fn handle_f6(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::F6) {
            self.randomise_circle_colors();
        }
    }

    fn handle_kp_plus(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::KpPlus) {
            self.add_new_circle();
        }
    }
    fn handle_kp_minus(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::KpMinus) {
            self.remove_oldest_circle();
        }
    }

    fn handle_plus(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::Equals) {
            if keyb.is_down(Scancode::LShift) || keyb.is_down(Scancode::RShift) {
                self.add_new_circle();
            }
        }
    }
    fn handle_minus(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::Minus) {
            self.remove_oldest_circle();
        }
    }
    fn handle_f(&mut self) {
        self.cycle_fill_mode();
    }
}

fn new_random_circle(fill_mode: FillMode) -> Circle {
    let mut rng = thread_rng();

    let c = Color::RGB(
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
    );

    let filled = match fill_mode {
        FillMode::None => false,
        FillMode::Some => rng.gen_range(0, 2) == 0,
        FillMode::All => true,
    };

    let mut circle = Circle {
        x: rng.gen_range(0, MAX_WIDTH + 1) as i16,
        y: rng.gen_range(0, MAX_HEIGHT + 1) as i16,
        radius: rng.gen_range(0, MAX_CIRCLE_RADIUS) as i16,
        color: c,
        filled: filled,
    };

    if circle.x - circle.radius <= 0 {
        circle.x = circle.radius;
    }
    if circle.x + circle.radius >= MAX_WIDTH as i16 {
        circle.x = MAX_WIDTH as i16 - circle.radius;
    }

    if circle.y - circle.radius <= 0 {
        circle.y = circle.radius;
    }
    if circle.y + circle.radius >= MAX_HEIGHT as i16 {
        circle.y = MAX_HEIGHT as i16 - circle.radius;
    }

    circle
}
