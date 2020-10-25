
use sdl2::rect::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonState {
    Released,
    Up,
    Pressed,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    cx: i32,
    cy: i32,
    cl: ButtonState,
    cm: ButtonState,
    cr: ButtonState,
    px: i32,
    py: i32,
    pl: ButtonState,
    pm: ButtonState,
    pr: ButtonState,
}

impl MouseState {
    pub fn new(events: &sdl2::EventPump) -> MouseState {
        let current: sdl2::mouse::MouseState = events.mouse_state();

        let mut ms = MouseState {
            cx: 0,
            cy: 0,
            cl: ButtonState::Up,
            cm: ButtonState::Up,
            cr: ButtonState::Up,
            px: 0,
            py: 0,
            pl: ButtonState::Up,
            pm: ButtonState::Up,
            pr: ButtonState::Up,
        };

        ms.cx = current.x();
        ms.cy = current.y();
        
        if current.left() {
            ms.cl = ButtonState::Down;
        }
        if current.middle() {
            ms.cm = ButtonState::Down;
        }
        if current.right() {
            ms.cr = ButtonState::Down;
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
            true => { ButtonState::Down }
            false => { ButtonState::Up }
        };

        self.cm = match current.left() {
            true => { ButtonState::Down }
            false => { ButtonState::Up }
        };

        self.cr = match current.right() {
            true => { ButtonState::Down }
            false => { ButtonState::Up }
        };
    }

    pub fn pos(&self) -> Point {
        Point::new(self.cx, self.cy)
    }

    pub fn x(&self) -> i32 {
        self.cx
    }

    pub fn y(&self) -> i32 {
        self.cy
    }

    pub fn left(self) -> ButtonState {
        self.cl
    }
    pub fn middle(self) -> ButtonState {
        self.cm
    }
    pub fn right(self) -> ButtonState {
        self.cr
    }

    pub fn is_up(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.cl == ButtonState::Up
            }
            MouseButton::Middle => { 
                self.cm == ButtonState::Up
            }
            MouseButton::Right => {
                self.cr == ButtonState::Up
            }
        }
    }

    pub fn is_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.cl == ButtonState::Down
            }
            MouseButton::Middle => { 
                self.cm == ButtonState::Down
            }
            MouseButton::Right => {
                self.cr == ButtonState::Down
            }
        }
    }

    pub fn on_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.pl == ButtonState::Up && self.cl == ButtonState::Down
            }
            MouseButton::Middle => { 
                self.pl == ButtonState::Up && self.cl == ButtonState::Down
            }
            MouseButton::Right => {
                self.pl == ButtonState::Up && self.cl == ButtonState::Down
            }
        }
    }

    pub fn on_up(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.pl == ButtonState::Down && self.cl == ButtonState::Up
            }
            MouseButton::Middle => { 
                self.pl == ButtonState::Down && self.cl == ButtonState::Up
            }
            MouseButton::Right => {
                self.pl == ButtonState::Down && self.cl == ButtonState::Up
            }
        }
    }

    pub fn state(&self, button: MouseButton) -> ButtonState {
        
        let (cstate, pstate) = match button {
            MouseButton::Left => (self.cl, self.pl),
            MouseButton::Middle => (self.cm, self.pm),
            MouseButton::Right => (self.cr, self.pr),
        };

        if cstate == ButtonState::Up && pstate == ButtonState::Down {
            ButtonState::Released
        } else if cstate == ButtonState::Up && pstate == ButtonState::Up {
            ButtonState::Up
        } else if cstate == ButtonState::Down && pstate == ButtonState::Up {
            ButtonState::Pressed
        } else  {
            ButtonState::Down
        } 
    }

}
