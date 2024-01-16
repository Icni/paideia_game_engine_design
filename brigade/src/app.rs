use nalgebra::Vector2;
use sdl2::image;
use legion::{World, Resources, Schedule};

pub struct App {
    pub title: String,
    pub window_size: Vector2<u32>,
}

impl App {
    pub fn run(self, mut start_schedule: Schedule, mut update_schedule: Schedule) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(&self.title, self.window_size.x, self.window_size.y)
            .build()
            .unwrap();
    }
}
