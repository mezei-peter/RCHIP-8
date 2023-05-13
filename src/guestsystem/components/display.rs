use sdl2::{Sdl, VideoSubsystem, video::Window, render::Canvas};

pub struct DisplayScreen<'a> {
    context: &'a Sdl,
    canvas: Canvas<Window>,
}

impl<'a> DisplayScreen<'a> {
    pub fn new(context: &Sdl) -> DisplayScreen {
        let video_subsystem: VideoSubsystem = context.video().unwrap();
        let window: Window = video_subsystem.window("RCHIP-8", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
        DisplayScreen{context, canvas}
    }
}
