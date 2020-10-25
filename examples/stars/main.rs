use rand::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use sdlgame::keyboard::KeyboardState;

const TITLE: &str = "Stars Demo";
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;
const STAR_COUNT: i32 = 20000;

#[derive(Debug, Clone)]
struct State {
    bgcolor: Color,
    stars: Vec<Star>,
}

impl State {
    fn new() -> State {
        State {
            bgcolor: Color::RGB(0, 0, 0),
            stars: Vec::new(),
        }
    }
}

fn main() -> Result<(), String> {
    let ctx_can = sdlgame::standard_800_600_canvas(TITLE, MAX_WIDTH, MAX_HEIGHT);
    let context = ctx_can.0;
    let mut canvas = ctx_can.1;

    let mut state = State::new();

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

        state.update_stars();

        let title = format!("{} - Star Count {}", TITLE, state.stars.len());
        canvas = sdlgame::set_window_title(canvas, &title);

        canvas.set_draw_color(state.bgcolor);
        canvas.clear();

        state.draw_stars(&mut canvas);

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

    fn update_stars(&mut self) {
        if self.stars.len() < STAR_COUNT as usize {
            self.stars.push(Star::new());
        }

        for star in self.stars.iter_mut() {
            let x = star.x + (star.speed_x); // * float64(w.FPS.Elapsed()))
            let y = star.y + (star.speed_y); // * float64(w.FPS.Elapsed()))
            if x < 0.0 || x > MAX_WIDTH as f64 || y < 0.0 || y > MAX_HEIGHT as f64 {
                let new_star = Star::new();
                star.x = new_star.x;
                star.y = new_star.y;
                star.speed_x = new_star.speed_x;
                star.speed_y = new_star.speed_y;
                star.color = new_star.color;
            } else {
                star.move_pos(x, y)
            }
        }
    }
    fn draw_stars(&self, canvas: &mut WindowCanvas) {
        for star in self.stars.iter() {
            canvas.set_draw_color(star.color);
            let p = Point::new(star.x as i32, star.y as i32);
            let _ = canvas.draw_point(p);
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Star {
    x: f64,
    y: f64,
    speed_x: f64,
    speed_y: f64,
    color: Color,
}

impl Star {
    fn new() -> Star {
        let mut rng = thread_rng();

        let angle = rng.gen_range(-3.14, 3.14) as f64;
        let speed_range = rng.gen_range(0.3, 0.9) as f64;
        let speed = 255.0 * speed_range.powi(2) as f64;
        let dx = angle.cos();
        let dy = angle.sin();

        let d = rng.gen_range(25, 125) as f64;

        let s = Star {
            x: (MAX_WIDTH / 2) as f64 + dx * d,
            y: (MAX_HEIGHT / 2) as f64 + dy * d,
            speed_x: speed * dx,
            speed_y: speed * dy,
            color: Color::RGB(
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
            ),
        };

        s
    }

    fn move_pos(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}
