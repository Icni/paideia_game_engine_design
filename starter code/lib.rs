// Put your module declarations here...
pub mod app;
pub mod assets;
pub mod rect;

pub mod ecs {
    pub use legion::*;
}

pub use ecs as legion;

pub use nalgebra::Vector2;
