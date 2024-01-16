use brigade::{app::App, Vector2, legion::{self, Schedule, system}};

fn main() {
    let app = App {
        title: "Testing!".to_string(),
        window_size: Vector2::new(800, 600),
    };

    let start = Schedule::builder()
        .build();

    let update = Schedule::builder()
        .build();

    app.run(start, update);
}
