use legion::{self, system};
use sdl2::render::WindowCanvas;

use crate::{assets::Assets, components::{Position, Sprite}};

pub struct Renderer {
    pub canvas: WindowCanvas,
}

#[system]
pub fn render_clear(#[resource] renderer: &mut Renderer) {
    renderer.canvas.clear();
}

#[system(for_each)]
pub fn render_sprites(sprite: &Sprite, position: &Position, #[resource] assets: &Assets, #[resource] renderer: &mut Renderer) {
    let texture = &assets.sdl_textures[sprite.texture.index];
    let src = sprite.source;
    let dst = sdl2::rect::Rect::new(
        position.value.x as i32,
        position.value.y as i32,
        sprite.texture.size.x,
        sprite.texture.size.y,
    );
    renderer.canvas.copy(texture, src, dst).expect("Could not render sprite.");
}

#[system]
pub fn render_present(#[resource] renderer: &mut Renderer) {
    renderer.canvas.present();
}
