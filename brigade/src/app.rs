use nalgebra::Vector2;
use legion::{systems::Runnable, Resources, Schedule, World, WorldOptions};
use sdl2::event::Event;

use crate::{assets::Assets, renderer::{render_clear_system, render_present_system, render_sprites_system, Renderer}};

/// Configurations for the application
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
        let mut event_pump = sdl_context.event_pump().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let mut world = World::new(WorldOptions::default());
        let mut resources = Resources::default();

        resources.insert(Renderer { canvas });
        resources.insert(Assets { texture_creator, sdl_textures: Vec::new() });

        start_schedule.execute(&mut world, &mut resources);

        let mut is_running = true;

        while is_running {
            for event in event_pump.poll_iter() {
                match &event {
                    Event::Quit { .. } => {
                        is_running = false;
                    }
                    _ => { }
                }
            }

            update_schedule.execute(&mut world, &mut resources);

            render_clear_system().run(&mut world, &mut resources);
            render_sprites_system().run(&mut world, &mut resources);
            render_present_system().run(&mut world, &mut resources);
        }
    }
}
