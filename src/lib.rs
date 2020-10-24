use std::collections::HashMap;

use sdl2::render::Canvas;
use sdl2::video::Window;
// use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;

pub fn set_window_title(canvas: Canvas<Window>, title: &str) -> Canvas<Window> {
    let mut c = canvas;
    {
        let w = c.window_mut();
        w.set_title(title).unwrap();
    }
    c
}

pub struct KeyboardState {
    previous: HashMap<Scancode, bool>,
    current: HashMap<Scancode, bool>,
}

impl KeyboardState {
    pub fn new(events: &sdl2::EventPump) -> KeyboardState {
        let mut state = KeyboardState {
            previous: HashMap::new(),
            current: HashMap::new(),
        };

        let keys = events.keyboard_state();

        for key in keys.scancodes() {
            state.current.insert(key.0, key.1);
            state.previous.insert(key.0, false);
        }
        state
    }

    pub fn update_state(&mut self, events: &sdl2::EventPump) {
        let keys = events.keyboard_state();

        for (key, state) in &self.current {
            self.previous.insert(*key, *state);
        }

        for key in keys.scancodes() {
            self.current.insert(key.0, key.1);
        }
    }

    pub fn is_down(&self, key: Scancode) -> bool {
        match self.current.get(&key) {
            Some(state) => *state,
            None => false,
        }
    }
    pub fn get_keystate(&self, key: Scancode) -> KeyState {
        let mut ks = KeyState {
            state: KeyStates::Up,
            shift: KeyStates::Up,
            alt: KeyStates::Up,
            ctrl: KeyStates::Up,
            left_shift: KeyStates::Up,
            right_shift: KeyStates::Up,
            left_alt: KeyStates::Up,
            right_alt: KeyStates::Up,
            left_ctrl: KeyStates::Up,
            right_ctrl: KeyStates::Up,
        };

        if self.is_down(key) {
            ks.state = KeyStates::Down
        };
        if self.is_down(Scancode::LShift) {
            ks.left_shift = KeyStates::Down;
            ks.shift = KeyStates::Down;
        };
        if self.is_down(Scancode::RShift) {
            ks.right_shift = KeyStates::Down;
            ks.shift = KeyStates::Down;
        };
        if self.is_down(Scancode::LAlt) {
            ks.left_alt = KeyStates::Down;
            ks.alt = KeyStates::Down;
        };
        if self.is_down(Scancode::RAlt) {
            ks.right_shift = KeyStates::Down;
            ks.alt = KeyStates::Down;
        };
        if self.is_down(Scancode::LCtrl) {
            ks.left_ctrl = KeyStates::Down;
            ks.ctrl = KeyStates::Down;
        };
        if self.is_down(Scancode::RCtrl) {
            ks.right_ctrl = KeyStates::Down;
            ks.alt = KeyStates::Down;
        };

        ks
    }
}

pub struct KeyState {
    pub state: KeyStates,
    pub shift: KeyStates,
    pub alt: KeyStates,
    pub ctrl: KeyStates,
    pub left_shift: KeyStates,
    pub left_alt: KeyStates,
    pub left_ctrl: KeyStates,
    pub right_shift: KeyStates,
    pub right_alt: KeyStates,
    pub right_ctrl: KeyStates,
}

#[derive(PartialEq)]
pub enum KeyStates {
    Up,
    Down,
}
