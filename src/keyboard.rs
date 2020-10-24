// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;

// use crate::GameCommand;

// pub struct 

// #[allow(dead_code)]
// pub struct Keyboard {
//     event_pump: sdl2::EventPump,
//     handle_keydown: Box<dyn FnMut(sdl2::event::Event) -> GameCommand>,
//     handle_keyup: Box<dyn FnMut(sdl2::event::Event) -> GameCommand>,
// }

// impl Keyboard {
//     pub fn new(event_pump: sdl2::EventPump) -> Keyboard {
//         Keyboard {
//             event_pump: event_pump,
//             // handle_keydown: Box::new(on_keydown_event_sync),
//             // handle_keyup: Box::new(on_keyup_event_sync),
//         }
//     }

//     // pub fn set_keydown_event(
//     //     &mut self,
//     //     f: impl FnMut(sdl2::event::Event) -> GameCommand + 'static,
//     // ) {
//     //     self.handle_keydown = Box::new(f);
//     // }

//     // pub fn clear_keydown_event(&mut self) {
//     //     self.handle_keydown = Box::new(on_keydown_event_sync)
//     // }

//     // pub fn set_keyup_event(&mut self, f: impl FnMut(sdl2::event::Event) -> GameCommand + 'static) {
//     //     self.handle_keyup = Box::new(f);
//     // }

//     // pub fn clear_keyup_event(&mut self) {
//     //     self.handle_keyup = Box::new(on_keyup_event_sync);
//     // }

//     pub fn poll(&mut self) -> GameCommand {
//         for event in self.event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. } |
//                 Event::KeyDown { keycode: sdl2::keycode::Q, .. }=> {
//                     return GameCommand::Quit
//                 }
//                 // Event::KeyDown { .. } => {
//                 //     return (self.handle_keydown)(event)
//                 // }
//                 // Event::KeyUp { .. } => {
//                 //     return (self.handle_keyup)(event)
//                 // }
//                 _ => {}
//             }
//         }
//         GameCommand::Continue
//     }
// }

// fn on_keydown_event_sync(_e: Event) -> GameCommand {
//     GameCommand::Continue
// }

// fn on_keyup_event_sync(_e: Event) -> GameCommand {
//     GameCommand::Continue
// }
