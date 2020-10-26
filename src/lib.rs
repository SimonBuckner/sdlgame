pub mod keyboard;
pub mod mouse;
pub mod shapes;

use sdl2::render::WindowCanvas;

pub trait Draw {
    fn draw(&self, canvas: &mut WindowCanvas);
}

pub fn standard_800_600_canvas(title: &str, width: u32, height: u32) -> (sdl2::Sdl, WindowCanvas) {
    let context = sdl2::init().expect("sdl2::init failed");
    let video_subsystem = context.video().expect("video subsytem init failed");

    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .opengl()
        .build()
        .expect("unable to create window");

    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("unable to create canvas");

    (context, canvas)
}

pub fn set_window_title(canvas: WindowCanvas, title: &str) -> WindowCanvas {
    let mut c = canvas;
    {
        let w = c.window_mut();
        w.set_title(title).unwrap();
    }
    c
}

pub fn calc_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {

    let x_dist = x1 - x2;
    let y_dist = y1 - y2;

    let dist = x_dist.pow(2) + y_dist.pow(2);

    (dist as f64).sqrt()
}
