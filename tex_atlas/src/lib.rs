extern crate glium;
extern crate image;
#[macro_use]
extern crate log;

use glium::backend::Facade;

pub type Texture2d = glium::texture::CompressedSrgbTexture2d;

#[derive(Debug)]
pub enum TextureAtlasError {
    IO(String),
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

impl From<std::io::Error> for TextureAtlasError {
    fn from(e: std::io::Error) -> Self {
        let s = format!("{:?}", e);
        TextureAtlasError::IO(s)
    }
}

pub struct TextureAtlas {
    texture: Texture2d,
    tile_size: (u32, u32),
    tile_count: (u32, u32),
    tex_ratio: [f32; 2],
    tex_coords: Vec<[f32; 2]>,
}

impl TextureAtlas {
    pub fn texture(&self) -> &Texture2d {
        &self.texture
    }

    pub fn texture_size(&self) -> (u32, u32) {
        let t = &self.texture;
        (t.get_width(), t.get_height().unwrap())
    }

    pub fn tile_size(&self) -> (u32, u32) {
        self.tile_size
    }

    pub fn tile_count(&self) -> (u32, u32) {
        self.tile_count
    }

    pub fn tex_ratio(&self) -> [f32; 2] {
        self.tex_ratio
    }

    pub fn tex_coords(&self) -> &Vec<[f32; 2]> {
        &self.tex_coords
    }
}

pub fn load<F: Facade>(display: &F,
                       image_file: &str,
                       tile_size: (u32, u32),
                       tile_count: (u32, u32),
                       color_mask: Option<[u8; 4]>) -> Result<TextureAtlas, TextureAtlasError> {

    let (img, dimensions) = {
        let mut img = try!(image::open(image_file)).to_rgba();
        let dimensions = img.dimensions();

        // apply the color mask
        if let Some(mask) = color_mask {
            use image::Pixel;
            let mask = &image::Rgba { data: mask };

            for x in 0..dimensions.0 {
                for y in 0..dimensions.1 {
                    let mut px = img.get_pixel_mut(x, y);
                    if px == mask {
                        px.apply(|_| 0);
                    }
                }
            }
        }

        (img, dimensions)
    };

    let img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.into_raw(), dimensions);
    let tex = try!(Texture2d::new(display, img));

    debug!("texture atlas is created: [texture: {:?}, tile_size: {:?}, tile_count: {:?}, color_mask: {:?}]",
           image_file, tile_size, tile_count, color_mask);

    let (cols, rows) = tile_count;
    let tex_ratio = [1.0 / cols as f32, 1.0 / rows as f32];

    Ok(TextureAtlas {
        texture: tex,
        tile_size: tile_size,
        tile_count: tile_count,
        tex_ratio: tex_ratio,
        tex_coords: {
            (0..cols*rows).map(|i| {
                let tx = (i % cols) as f32 * tex_ratio[0];
                let ty = (i / cols) as f32 * tex_ratio[1];
                [tx, ty]
            }).collect()
        },
    })
}
