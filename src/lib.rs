pub mod keyboard;
pub mod mouse;

pub fn standard_800_600_canvas(title: &str, width: u32, height: u32) -> (sdl2::Sdl, sdl2::render::WindowCanvas) {
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

pub fn set_window_title(canvas: sdl2::render::WindowCanvas, title: &str) -> sdl2::render::WindowCanvas {
    let mut c = canvas;
    {
        let w = c.window_mut();
        w.set_title(title).unwrap();
    }
    c
}

