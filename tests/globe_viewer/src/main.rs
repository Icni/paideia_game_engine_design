use std::path::Path;

use brigade::{legion, App, Vector2, Game, Sprite, Position, Velocity, Input, Keycode, Assets, ecs::{system, Schedule, systems::CommandBuffer}, EventHandler, vector2, rect};

const SPEED: f32 = 100.0;

struct GlobeViewer;

impl Game for GlobeViewer {
    fn start(&self) -> Schedule {
        Schedule::builder()
            .add_thread_local(init_system())
            .build()
    }

    fn update(&self) -> Schedule {
        Schedule::builder()
            .add_system(input_system())
            .build()
    }
}

#[system(for_each)]
fn input(vel: &mut Velocity, #[resource] event_handler: &EventHandler) {
    for input in &event_handler.inputs {
        if let Input::KeyDown { keycode: Some(key), .. } = input {
            match key {
                Keycode::W => vel.0.y = -SPEED,
                Keycode::S => vel.0.y = SPEED,
                Keycode::A => vel.0.x = -SPEED,
                Keycode::D => vel.0.x = SPEED,
                _ => {}
            };
        } else if let Input::KeyUp { keycode: Some(key), .. } = input {
            match key {
                Keycode::W => vel.0.y = 0.,
                Keycode::A => vel.0.x = 0.,
                Keycode::S => vel.0.y = 0.,
                Keycode::D => vel.0.x = 0.,
                _ => {}
            };
        }
    }
}

#[system]
fn init(cmd: &mut CommandBuffer, #[resource] assets: &mut Assets) {
    let texture = assets.load_texture(Path::new("tests/globe_viewer/assets/textures/globe.png")).unwrap();
    cmd.push((Position(vector2!(0., 0.)), Velocity(vector2!(0., 0.)), Sprite {
        texture: texture.clone(),
        src_rect: rect!([0, 0], texture.size()),
        scale: vector2!(0.25),
    },));
}

fn main() {
    let app = App::new(
        GlobeViewer,
        "Globe Viewer".to_string(),
        Vector2::new(800, 800)
    );

    app.run();
}
