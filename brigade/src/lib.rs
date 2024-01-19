// Put your module declarations here...
pub mod app;
pub mod assets;
pub mod components;
pub mod rect;
mod renderer;

pub mod ecs {
    pub use legion::*;
}

pub use ecs as legion;

pub use nalgebra::Vector2;
