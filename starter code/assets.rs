use std::path::Path;

use anyhow::Result;
use nalgebra::Vector2;
use sdl2::{pixels::PixelFormatEnum, render::TextureCreator, surface::Surface, video::WindowContext};

use crate::{components::Sprite, rect::Rect};

/// Alternative to sdl2::image::LoadTexture
trait LoadTexture {
    fn load_texture<P: AsRef<Path>>(&self, path: P) -> Result<sdl2::render::Texture>;
}

// Violà! Another library download avoided!
impl LoadTexture for TextureCreator<WindowContext> {
    fn load_texture<P: AsRef<Path>>(&self, path: P) -> Result<sdl2::render::Texture> {
        let mut image = image::open(path)?.into_rgba8();
        let width = image.width();
        let height = image.height();
        let pitch: u32 = image.sample_layout().height_stride.try_into()?;
        let surface = Surface::from_data(
            &mut image,
            width,
            height,
            pitch,
            PixelFormatEnum::RGBA32,
        ).map_err(anyhow::Error::msg)?;
        Ok(self.create_texture_from_surface(surface)?)
    }
}
