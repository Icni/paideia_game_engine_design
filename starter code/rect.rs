use nalgebra::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect<T> {
    pub position: Vector2<T>,
    pub size: Vector2<T>,
}

impl<T> Rect<T> {
    pub fn new(position: Vector2<T>, size: Vector2<T>) -> Self {
        Self {
            position,
            size,
        }
    }
}

impl From<Rect<i32>> for Option<sdl2::rect::Rect> {
    fn from(value: Rect<i32>) -> Self {
        Some(sdl2::rect::Rect::new(
            value.position.x,
            value.position.y,
            value.size.x.try_into().unwrap_or(0),
            value.size.y.try_into().unwrap_or(0),
        ))
    }
}

impl From<Rect<u32>> for Option<sdl2::rect::Rect> {
    fn from(value: Rect<u32>) -> Self {
        Some(sdl2::rect::Rect::new(
            value.position.x.try_into().unwrap_or(0),
            value.position.y.try_into().unwrap_or(0),
            value.size.x,
            value.size.y,
        ))
    }
}
