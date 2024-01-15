use nalgebra::Vector2;
use sdl2::image;
use legion::{World, Resources, Schedule};

pub struct App {
    pub title: String,
    pub window_size: Vector2<u32>,
}

impl App {
    pub fn run(self, mut start_schedule: Schedule, mut update_schedule: Schedule) {
        // This is your app's run function!
        // TODO
    }
}
