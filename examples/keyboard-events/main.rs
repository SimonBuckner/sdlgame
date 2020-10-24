extern crate sdlgame;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::keyboard::Mod;

// use sdlgame::*;

#[allow(dead_code)]
struct State {
    // Sdl State
    // context: Sdl,
    // canvas: Canvas<Window>,

    // Game state
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
        .build()
        .expect("unable to create canvas");
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

        canvas = sdlgame::set_window_title(canvas, "Updated window title");

        canvas.set_draw_color(state.bgcolor);
        canvas.clear();
        canvas.present();
    }
    Ok(())
}
