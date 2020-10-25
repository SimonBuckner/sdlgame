extern crate sdlgame;

use rand::prelude::*;

use sdl2::gfx::primitives::DrawRenderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdlgame::keyboard::KeyboardState;

const TITLE: &str = "Mouse State Demo";
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;
const INITIAL_CIRCLES: usize = 50;
const MAX_CIRCLE_RADIUS: u32 = 100;

#[derive(Debug)]
struct Line {
    x: i32,
    y: i32,
    color: Color,
}

#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
    filled: bool,
}

// #[allow(dead_code)]

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
    lines: Vec<Line>,
    fill_mode: FillMode,
    is_drawing: bool,
    start_x: i32,
    start_y: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum FillMode {
    None,
    Some,
    All,
}

fn main() -> Result<(), String> {
    let ctx_can = sdlgame::standard_800_600_canvas(TITLE, MAX_WIDTH, MAX_HEIGHT);
    let mut context = ctx_can.0;
    let mut canvas = ctx_can.1;

    let mut state = State {
        bgcolor: Color::RGB(0, 0, 0),
        lines: Vec::new(),
        fill_mode: FillMode::None,
        is_drawing: false,
        start_x: -1,
        start_y: -1,
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

        canvas.set_draw_color(state.bgcolor);
        canvas.clear();

        state.draw_circles(&mut canvas);

        canvas.present();
    }
    Ok(())
}

impl State {

}
