use <replace with your engine name>::{legion, ecs::{Schedule, system, systems::CommandBuffer}, components::{Velocity, Sprite, Position}, event::EventHandler, input::{Input, Keycode}, assets::Assets, rect::Rect, app::App, Vector2};

const SPEED: f32 = 100.0;

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
    let texture = assets.load_texture("globe.png").unwrap();
    cmd.push((Position::new(0., 0.), Velocity(Vector2::new(0., 0.)), Sprite {
        texture: texture.clone(),
        src_rect: Rect::new(Vector2::new(0, 0), texture.size()),
        scale: Vector2::new(0.25, 0.25),
    },));
}

fn main() {
    let app = App {
        title: "Globe Viewer".to_string(),
        window_size: Vector2::new(800, 800),
    };

    let start = Schedule::builder()
        .add_thread_local(init_system())
        .build();
    let update = Schedule::builder()
        .add_system(input_system())
        .build();

    app.run(start, update);
}
