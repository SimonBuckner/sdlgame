// use rand::prelude::*;

// use sdl2::gfx::primitives::DrawRenderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use sdlgame::keyboard::KeyboardState;
use sdlgame::mouse::{ButtonState, MouseButton, MouseState};
use sdlgame::shapes::{Circle, Line, Rectangle};
use sdlgame::Draw;

const TITLE: &str = "Mouse State Demo. L/S/C Draws Lines, Squares & Circles with left mouse button";
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;

#[derive(Debug, Copy, Clone, PartialEq)]
enum DrawMode {
    None,
    Line,
    Rectangle,
    Circle,
}

// #[derive(Debug)]
struct State {
    bgcolor: Color,
    shapes: Vec<Box<dyn Draw>>,
    // fill_mode: FillMode,
    is_drawing: bool,
    start_x: i32,
    start_y: i32,
    drawing: DrawMode,
}

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum FillMode {
//     None,
//     Some,
//     All,
// }

fn main() -> Result<(), String> {
    let ctx_can = sdlgame::standard_800_600_canvas(TITLE, MAX_WIDTH, MAX_HEIGHT);
    let context = ctx_can.0;
    let mut canvas = ctx_can.1;

    let mut state = State::new();

    let mut event_pump = context.event_pump()?;

    let mut keyb = KeyboardState::new(&event_pump);
    let mut mouse = MouseState::new(&event_pump);

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
        keyb.update(&event_pump);
        mouse.update(&event_pump);

        state.handle_l(&keyb);
        state.handle_s(&keyb);
        state.handle_c(&keyb);
        
        canvas.set_draw_color(state.bgcolor);
        canvas.clear();

        match mouse.state(MouseButton::Left) {
            ButtonState::Pressed => {
                state.start_x = mouse.x();
                state.start_y = mouse.y();
                state.start_shape(&mouse);
                state.is_drawing = true;
            }
            ButtonState::Released => {
                state.add_shape(&mouse);
                state.is_drawing = false;
            }
            ButtonState::Down => {
                state.draw_current(&mouse, &mut canvas);
            }
            _ => {}
        }

        state.draw_shapes(&mut canvas);

        canvas.present();
    }
    Ok(())
}

impl State {
    fn new() -> State {
        State {
            bgcolor: Color::RGB(0, 0, 0),
            shapes: Vec::new(),
            // fill_mode: FillMode::None,
            is_drawing: false,
            start_x: -1,
            start_y: -1,
            drawing: DrawMode::None,
        }
    }

    fn start_shape(&mut self, mouse: &MouseState) {
        match self.drawing {
            DrawMode::None => {}
            DrawMode::Line => {
                let mut line = Line::new();
                line.x1 = self.start_x;
                line.y1 = self.start_y;
                line.x2 = mouse.x();
                line.y2 = mouse.y();
            }
            DrawMode::Rectangle => {
                let mut rect = Rectangle::new();
                rect.x = self.start_x;
                rect.y = self.start_y;
                rect.width = 0;
                rect.height = 0;
            }
            DrawMode::Circle => {
                let mut circle = Circle::new();
                circle.x = mouse.x();
                circle.y = mouse.y();
                circle.radius = 0;
            }
        }
    }

    fn draw_current(&self, mouse: &MouseState, canvas: &mut WindowCanvas) {
        match self.drawing {
            DrawMode::None => {}
            DrawMode::Line => {
                let mut line = Line::new();
                line.x1 = self.start_x;
                line.y1 = self.start_y;
                line.x2 = mouse.x();
                line.y2 = mouse.y();
                line.draw(canvas);
            }
            DrawMode::Rectangle => {
                let mut rect = Rectangle::new();
                rect.x = self.start_x;
                rect.y = self.start_y;
                rect.width = mouse.x() - self.start_x;
                rect.height = mouse.y() - self.start_y;
                rect.draw(canvas);
            }
            DrawMode::Circle => {
                let mut circle = Circle::new();
                circle.x = self.start_x;
                circle.y = self.start_y;
                circle.radius = sdlgame::calc_distance(circle.x, circle.y, mouse.x(), mouse.y()) as i32;
                circle.draw(canvas);
            }
        }
    }

    fn add_shape(&mut self, mouse: &MouseState) {
        match self.drawing {
            DrawMode::None => {}
            DrawMode::Line => {
                let mut line = Line::new();
                line.x1 = self.start_x;
                line.y1 = self.start_y;
                line.x2 = mouse.x();
                line.y2 = mouse.y();
                self.shapes.push(Box::new(line));
            }
            DrawMode::Rectangle => {
                let mut rect = Rectangle::new();
                rect.x = self.start_x;
                rect.y = self.start_y;
                rect.width = mouse.x() - self.start_x;
                rect.height = mouse.y() - self.start_y;
                self.shapes.push(Box::new(rect));
            }
            DrawMode::Circle => {
                let mut circle = Circle::new();
                circle.x = self.start_x;
                circle.y = self.start_y;
                circle.radius = sdlgame::calc_distance(circle.x, circle.y, mouse.x(), mouse.y()) as i32;
                self.shapes.push(Box::new(circle));
            }
        }
    }

    fn draw_shapes(&self, canvas: &mut WindowCanvas) {
        for shape in self.shapes.iter() {
            shape.draw(canvas);
        }
    }

    fn handle_l(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::L) {
            self.drawing = DrawMode::Line;
        }
    }

    fn handle_s(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::S) {
            self.drawing = DrawMode::Rectangle;
        }
    }

    fn handle_c(&mut self, keyb: &KeyboardState) {
        if keyb.is_down(Scancode::C) {
            self.drawing = DrawMode::Circle;
        }
    }
}
