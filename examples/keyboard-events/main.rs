extern crate sdlgame;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::keyboard::Mod;

// use sdlgame::*;

const TITLE: &str = "Keyboard Events Demo - R,G,B increase Red,Green,Blue. Shift+R/G/B descreases";
const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 600;

#[allow(dead_code)]
struct State {
    // Sdl State
    // context: Sdl,
    // canvas: Canvas<Window>,

    // Game state
    bgcolor: Color,
}

fn main() -> Result<(), String> {
    let ctx_can = sdlgame::standard_800_600_canvas(TITLE, MAX_WIDTH, MAX_HEIGHT);
    let context = ctx_can.0;
    let mut canvas = ctx_can.1;

    let mut state = State {
        // context: context,
        // canvas: canvas,
        bgcolor: Color::RGB(0, 0, 0),
    };

    let mut event_pump = context.event_pump()?;
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
                Event::KeyDown { keycode:Some(Keycode::R), keymod, ..} => {
                    if keymod == Mod::LSHIFTMOD || keymod == Mod::RSHIFTMOD {
                        if state.bgcolor.r != 0 {
                            state.bgcolor.r -= 1;
                        }
                    } else {
                        if state.bgcolor.r < 255 {
                            state.bgcolor.r += 1;
                        }
                    }
                }
                Event::KeyDown { keycode:Some(Keycode::G), keymod, ..} => {
                    if keymod == Mod::LSHIFTMOD || keymod == Mod::RSHIFTMOD {
                        if state.bgcolor.g != 0 {
                            state.bgcolor.g -= 1;
                        }
                    } else {
                        if state.bgcolor.g < 255 {
                            state.bgcolor.g += 1;
                        }
                    }
                }
                Event::KeyDown { keycode:Some(Keycode::B), keymod, ..} => {
                    if keymod == Mod::LSHIFTMOD || keymod == Mod::RSHIFTMOD {
                        if state.bgcolor.b != 0 {
                            state.bgcolor.b -= 1;
                        }
                    } else {
                        if state.bgcolor.b < 255 {
                            state.bgcolor.b += 1;
                        }
                    }
                }                
                _ => {}
            }
        }

        let title = format!("{} - R({}) G({}) B({})", TITLE, state.bgcolor.r, state.bgcolor.g,state.bgcolor.b);
        canvas = sdlgame::set_window_title(canvas, &title);

        canvas.set_draw_color(state.bgcolor);
        canvas.clear();
        canvas.present();
    }
    Ok(())
}
