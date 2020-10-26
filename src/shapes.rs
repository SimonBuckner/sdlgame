use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::Draw;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub color: Color,
}

impl Line {
    #[allow(dead_code)]
    pub fn new() -> Line {
        Line {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            color: Color::RGB(255, 255, 255),
        }
    }
}

impl Draw for Line {
    fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        let p1 = Point::new(self.x1, self.y1);
        let p2 = Point::new(self.x2, self.y2);
        let _ = canvas.draw_line(p1, p2);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub filled: bool,
}

impl Rectangle {
    #[allow(dead_code)]
    pub fn new() -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            color: Color::RGB(255, 255, 255),
            filled: false,
        }
    }
}

impl Draw for Rectangle {
    fn draw(&self, canvas: &mut WindowCanvas) {
        let (x, width) = if self.width >= 0 {
            (self.x, self.width)
        } else {
            ((self.x + self.width), (self.width * -1))
        };

        let (y, height) = if self.height >= 0 {
            (self.y, self.height)
        } else {
            ((self.y + self.height), (self.height * -1))
        };

        canvas.set_draw_color(self.color);
        let r = Rect::new(x, y, width as u32, height as u32);
        if self.filled {
            let _ = canvas.fill_rect(r);
        } else {
            let _ = canvas.draw_rect(r);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub color: Color,
    pub filled: bool,
}

impl Circle {
    #[allow(dead_code)]
    pub fn new() -> Circle {
        Circle {
            x: 0,
            y: 0,
            radius: 0,
            color: Color::RGB(255, 255, 255),
            filled: false,
        }
    }
}

impl Draw for Circle {
    fn draw(&self, canvas: &mut WindowCanvas) {
        if self.filled {
            let _ =
                canvas.filled_circle(self.x as i16, self.y as i16, self.radius as i16, self.color);
        } else {
            let _ = canvas.circle(self.x as i16, self.y as i16, self.radius as i16, self.color);
        }
    }
}
