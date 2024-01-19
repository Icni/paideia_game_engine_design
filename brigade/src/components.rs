use nalgebra::Vector2;

use crate::{assets::Texture, rect::Rect};

pub struct Position {
    pub value: Vector2<f32>,
}

pub struct Sprite {
    pub texture: Texture,
    pub source: Rect<u32>,
    pub scale: Vector2<f32>,
}
