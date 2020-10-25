
use sdl2::rect::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonStates {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    cx: i32,
    cy: i32,
    cl: ButtonStates,
    cm: ButtonStates,
    cr: ButtonStates,
    px: i32,
    py: i32,
    pl: ButtonStates,
    pm: ButtonStates,
    pr: ButtonStates,
}

impl MouseState {
    pub fn new(events: &sdl2::EventPump) -> MouseState {
        let current: sdl2::mouse::MouseState = events.mouse_state();

        let mut ms = MouseState {
            cx: 0,
            cy: 0,
            cl: ButtonStates::Up,
            cm: ButtonStates::Up,
            cr: ButtonStates::Up,
            px: 0,
            py: 0,
            pl: ButtonStates::Up,
            pm: ButtonStates::Up,
            pr: ButtonStates::Up,
        };

        ms.cx = current.x();
        ms.cy = current.y();
        
        if current.left() {
            ms.cl = ButtonStates::Down;
        }
        if current.middle() {
            ms.cm = ButtonStates::Down;
        }
        if current.right() {
            ms.cr = ButtonStates::Down;
        }

        ms
    }

    pub fn update(&mut self, events: &sdl2::EventPump) {
        let current: sdl2::mouse::MouseState = events.mouse_state();

        self.px = self.cx;
        self.py = self.cy;
        self.pl = self.cl;
        self.pm = self.cm;
        self.pr = self.cr;

        self.cx = current.x();
        self.cy = current.y();

        self.cl = match current.left() {
            true => { ButtonStates::Down }
            false => { ButtonStates::Up }
        };

        self.cm = match current.left() {
            true => { ButtonStates::Down }
            false => { ButtonStates::Up }
        };

        self.cr = match current.right() {
            true => { ButtonStates::Down }
            false => { ButtonStates::Up }
        };
    }

    pub fn get_pos(&self) -> Point {
        Point::new(self.cx, self.cy)
    }

    pub fn left(self) -> ButtonStates {
        self.cl
    }
    pub fn middle(self) -> ButtonStates {
        self.cm
    }
    pub fn right(self) -> ButtonStates {
        self.cr
    }

    pub fn is_up(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.cl == ButtonStates::Up
            }
            MouseButton::Middle => { 
                self.cm == ButtonStates::Up
            }
            MouseButton::Right => {
                self.cr == ButtonStates::Up
            }
        }
    }

    pub fn is_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.cl == ButtonStates::Down
            }
            MouseButton::Middle => { 
                self.cm == ButtonStates::Down
            }
            MouseButton::Right => {
                self.cr == ButtonStates::Down
            }
        }
    }

    pub fn on_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.pl == ButtonStates::Up && self.cl == ButtonStates::Down
            }
            MouseButton::Middle => { 
                self.pl == ButtonStates::Up && self.cl == ButtonStates::Down
            }
            MouseButton::Right => {
                self.pl == ButtonStates::Up && self.cl == ButtonStates::Down
            }
        }
    }

    pub fn on_up(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.pl == ButtonStates::Down && self.cl == ButtonStates::Up
            }
            MouseButton::Middle => { 
                self.pl == ButtonStates::Down && self.cl == ButtonStates::Up
            }
            MouseButton::Right => {
                self.pl == ButtonStates::Down && self.cl == ButtonStates::Up
            }
        }
    }

}
