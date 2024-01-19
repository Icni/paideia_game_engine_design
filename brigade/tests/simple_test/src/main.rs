use std::path::Path;

use brigade::{app::App, assets::Assets, components::{Position, Sprite}, ecs::systems::CommandBuffer, legion::{self, Schedule, system}, rect::Rect, Vector2};

const IMAGE_PATH: &str = "tests/simple_test/assets/textures";

#[system]
fn init(cmd: &mut CommandBuffer, #[resource] assets: &mut Assets) {
    let texture = assets.load_texture(&Path::new(IMAGE_PATH));
    let source = Rect::new(Vector2::new(0, 0), texture.size);
    let scale = Vector2::new(0.25, 0.25);
    cmd.push((
        Position { value: Vector2::new(400.0, 600.0) },
        Sprite {
            texture,
            source,
            scale,
        },
    ));
}

fn main() {
    let app = App {
        title: "Testing!".to_string(),
        window_size: Vector2::new(800, 600),
    };

    let start = Schedule::builder()
        .add_thread_local(init_system())
        .build();

    let update = Schedule::builder()
        .build();

    app.run(start, update);
}
