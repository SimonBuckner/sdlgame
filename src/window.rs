// use std::{thread, time};
// use std::sync::{Arc, Mutex};

// use sdl2::event::Event;
// use sdl2::pixels::Color;
// use sdl2::keyboard::Keycode;

// use crate::{Game}; //, GameCommand};

// pub struct Window {
//     game: Game,
//     canvas: sdl2::render::WindowCanvas,

//     background_color: Color,
//     run_loop_sleep: u64,
// }

// impl Window {
//     pub fn new(
//         game: Game,
//         title: &str,
//         width: u32,
//         height: u32,
//     ) -> Result<Window, Box<dyn std::error::Error>> {
//         let sdlw = game
//             .video_subsystem
//             .window(title, width, height)
//             .position_centered()
//             .build()?;

//         let sdlc = sdlw.into_canvas().build()?;

//         let w = Window {
//             game: game,
//             canvas: sdlc,
//             background_color: Color::RGB(255, 255, 255),
//             run_loop_sleep: 8,
//         };
        
//         Ok(w)
//     }

//     pub fn run(&mut self) -> Result<(), String> {
//         let mut event_pump = self.game.context.event_pump()?;

//         'running: loop {
//             // Handle keyboard events
//             for event in event_pump.poll_iter() {
//                 // let mut gc = GameCommand::Continue;
//                 match event {
//                     Event::Quit { .. } |
//                     Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
//                         break 'running;
//                     },
//                     // Event::KeyDown { .. } => {
//                     //     gc = (self.handle_keydown)(event);
//                     // }
//                     _ => {},
//                 }
//                 // if let GameCommand::Quit = gc {
//                 //     break 'running;
//                 // }
//             }

//             self.notify_update();

//             self.canvas.set_draw_color(self.background_color);
//             self.canvas.clear();

//             self.canvas.present();
//             thread::sleep(time::Duration::from_millis(self.run_loop_sleep));
//         }
//         Ok(())
//     }

//     pub fn set_title(&mut self, title: &str) {
//         self.canvas.window_mut().set_title(title).unwrap();
//     }

//     pub fn set_background_color(&mut self, color: Color) {
//         self.background_color = color;
//     }

//     pub fn set_run_loop_sleep(&mut self, ms: u64) {
//         self.run_loop_sleep = ms;
//     }
// }
