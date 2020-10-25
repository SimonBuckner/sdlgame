use std::collections::HashMap;

use sdl2::keyboard::Scancode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyStates {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub struct KeyboardState {
    previous: HashMap<Scancode, bool>,
    current: HashMap<Scancode, bool>,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyState {
    state: KeyStates,
    shift: KeyStates,
    alt: KeyStates,
    ctrl: KeyStates,
    left_shift: KeyStates,
    left_alt: KeyStates,
    left_ctrl: KeyStates,
    right_shift: KeyStates,
    right_alt: KeyStates,
    right_ctrl: KeyStates,
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

    pub fn update(&mut self, events: &sdl2::EventPump) {
        let keys = events.keyboard_state();

        for (key, state) in &self.current {
            self.previous.insert(*key, *state);
        }

        for key in keys.scancodes() {
            self.current.insert(key.0, key.1);
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

    pub fn is_down(&self, key: Scancode) -> bool {
        match self.current.get(&key) {
            Some(state) => *state,
            None => false,
        }
    }

    pub fn is_up(&self, key: Scancode) -> bool {
        match self.current.get(&key) {
            Some(state) => *state,
            None => false,
        }
    }

    pub fn on_down(&self, key: Scancode) -> bool {
        let cs = match self.current.get(&key) {
            Some(state) => *state,
            None => false,
        };
        let ls = match self.previous.get(&key) {
            Some(state) => *state,
            None => false,
        };
        ls == false && cs == true
    }

    pub fn on_up(&self, key: Scancode) -> bool {
        let cs = match self.current.get(&key) {
            Some(state) => *state,
            None => false,
        };
        let ls = match self.previous.get(&key) {
            Some(state) => *state,
            None => false,
        };
        ls == true && cs == false
    }
}
