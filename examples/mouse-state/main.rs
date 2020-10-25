// use rand::prelude::*;

// use sdl2::gfx::primitives::DrawRenderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use sdlgame::keyboard::KeyboardState;
use sdlgame::mouse::{ButtonState,MouseButton, MouseState};

mod shapes;
use shapes::Line;

const TITLE: &str = "Mouse State Demo";
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;


#[derive(Debug)]
struct State {
    bgcolor: Color,
    lines: Vec<Line>,
    // fill_mode: FillMode,
    is_drawing: bool,
    start_x: i32,
    start_y: i32,
    drawing_line: Line,
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

        canvas.set_draw_color(state.bgcolor);
        canvas.clear();

        match mouse.state(MouseButton::Left) {
            ButtonState::Pressed => {
                state.is_drawing = true;
                state.start_x = mouse.x();
                state.start_y = mouse.y();
            }
            ButtonState::Released => {
                let mut line = Line::new();
                line.x1 = state.start_x;
                line.y1 = state.start_y;
                line.x2 = mouse.x();
                line.y2 = mouse.y();
                state.lines.push(line);
                state.is_drawing = false;
            }
            ButtonState::Down => {
                state.drawing_line.x1 = state.start_x;
                state.drawing_line.y1 = state.start_y;
                state.drawing_line.x2 = mouse.x();
                state.drawing_line.y2 = mouse.y();
                state.drawing_line.draw(&mut canvas);
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
            lines: Vec::new(),
            // fill_mode: FillMode::None,
            is_drawing: false,
            start_x: -1,
            start_y: -1,
            drawing_line: Line::new(),
        }
    }

    fn draw_shapes(&self, canvas: &mut WindowCanvas) {
        for line in self.lines.iter() {
            line.draw(canvas);
        }
    }
}
