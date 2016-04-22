extern crate glium;
extern crate image;

use std::path::Path;
use glium::backend::Facade;

pub type Texture2d = glium::texture::CompressedSrgbTexture2d;

#[derive(Debug)]
pub enum TextureAtlasError {
    Image(String),
    Texture(String),
}

impl From<image::ImageError> for TextureAtlasError {
    fn from(e: image::ImageError) -> Self {
        let s = format!("{:?}", e);
        TextureAtlasError::Image(s)
    }
}

impl From<glium::texture::TextureCreationError> for TextureAtlasError {
    fn from(e: glium::texture::TextureCreationError) -> Self {
        let s = format!("{:?}", e);
        TextureAtlasError::Texture(s)
    }
}

pub struct TextureAtlas {
    tex: Texture2d,
    tex_size: (u32, u32),
    tile_count: (u32, u32),
}

impl TextureAtlas {
    pub fn texture(&self) -> &Texture2d {
        &self.tex
    }

    pub fn texture_size(&self) -> (u32, u32) {
        self.tex_size
    }

    pub fn tile_count(&self) -> (u32, u32) {
        self.tile_count
    }

    pub fn ratio(&self) -> [f32; 2] {
        let (rows, cols) = self.tile_count();
        [1.0 / rows as f32, 1.0 / cols as f32]
    }
}

pub fn load<F>(display: &F, path: &Path) -> Result<TextureAtlas, TextureAtlasError> where F: Facade {
    let img = try!(image::open(path)).to_rgba();
    let dimensions = img.dimensions();

    let img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.into_raw(), dimensions);
    let tex = try!(Texture2d::new(display, img));

    Ok(TextureAtlas {
        tex: tex,
        tex_size: dimensions,
        tile_count: (16, 16),
    })
}
