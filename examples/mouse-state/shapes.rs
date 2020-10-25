use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;
use sdl2::rect::Rect;

#[derive(Debug, Copy, Clone)]
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
        Line{
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            color: Color::RGB(255,255,255),
        }
    }

    #[allow(dead_code)]
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        let p1 = Point::new(self.x1, self.y1);
        let p2 = Point::new(self.x2, self.y2);
        let _= canvas.draw_line(p1, p2);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Box {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
    filled: bool,
}

impl Box {
    #[allow(dead_code)]
    pub fn new() -> Box {
        Box{
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            color: Color::RGB(0,0,0),
            filled: false,
        }
    }

    #[allow(dead_code)]
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        let r = Rect::new(self.x, self.y, self.width as u32, self.height as u32);
        if self.filled {
            let _ = canvas.fill_rect(r);
        } else {
            let _ = canvas.draw_rect(r);
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Circle {
    x: i16,
    y: i16,
    radius: i16,
    color: Color,
    filled: bool,
}

impl Circle {
    #[allow(dead_code)]
    pub fn new() -> Circle {
        Circle{
            x: 0,
            y: 0,
            radius: 0,
            color: Color::RGB(0,0,0),
            filled: false,
        }
    }

    #[allow(dead_code)]
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        if self.filled {
            let _ = canvas.filled_circle(self.x as i16, self.y as i16, self.radius as i16, self.color);
        } else {
            let _ = canvas.circle(self.x as i16, self.y as i16, self.radius as i16, self.color);
        }
    }
}