#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate glium;
extern crate image;
extern crate toml;
#[macro_use]
extern crate log;

use std::path::Path;
use glium::backend::Facade;

pub type Texture2d = glium::texture::CompressedSrgbTexture2d;

#[derive(Debug)]
pub enum TextureAtlasError {
    IO(String),
    Parse(String),
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

    pub fn ratio(&self) -> [f32; 2] {
        let (rows, cols) = self.tile_count();
        [1.0 / rows as f32, 1.0 / cols as f32]
    }
}

pub fn load<F: Facade>(display: &F, path: &Path) -> Result<TextureAtlas, TextureAtlasError> {
    use std::fs::File;
    use std::io::Read;

    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));


    let mut p = toml::Parser::new(&s);
    match p.parse() {
        Some(table) => {
            use toml_utils;

            let image_file = {
                let k = "file";
                try!(toml_utils::required_string(&table, k))
            };

            let tile_size = {
                let k = "tile_size";
                try!(toml_utils::vec_to_pair(k, try!(toml_utils::required_vec_of_u32(&table, k))))
            };

            let tile_count = {
                let k = "tile_count";
                try!(toml_utils::vec_to_pair(k, try!(toml_utils::required_vec_of_u32(&table, k))))
            };

            let color_mask = {
                let k = "color_mask";
                match try!(toml_utils::optional_vec_of_u32(&table, k)) {
                    Some(val) => Some(try!(toml_utils::vec_to_rbga8(k, val))),
                    _ => None
                }
            };

            let (img, dimensions) = {
                let mut img = try!(image::open(image_file)).to_rgba();
                let dimensions = img.dimensions();

                // apply the color mask
                if let Some(ref mask) = color_mask {
                    use image::Pixel;

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

            debug!("texture atlas is created from {:?}: [texture: {:?}, tile_size: {:?}, tile_count: {:?}, color_mask: {:?}]", path, image_file, tile_size, tile_count, color_mask);

            Ok(TextureAtlas {
                texture: tex,
                tile_size: tile_size,
                tile_count: tile_count,
            })
        },
        _ => {
            let s = {
                let mut s = String::new();
                for e in &p.errors {
                    s.push_str(&format!("{:?}", e));
                    s.push_str("\n");
                }
                s
            };

            Err(TextureAtlasError::Parse(s))
        },
    }
}

mod toml_utils {
    use toml::{Table, Value};
    use super::TextureAtlasError;
    use super::image;

    // TODO deal with this mess

    fn required_field<'a>(t: &'a Table, k: &str) -> Result<&'a Value, TextureAtlasError> {
        match t.get(k) {
            Some(v) => Ok(v),
            _ => Err(TextureAtlasError::Parse(format!("Missing value of '{}'", k))),
        }
    }

    fn optional_field<'a>(t: &'a Table, k: &str) -> Option<&'a Value> {
        t.get(k)
    }

    pub fn required_string<'a>(t: &'a Table, k: &str) -> Result<&'a String, TextureAtlasError> {
        let val = try!(required_field(t, k));
        match *val {
            Value::String(ref s) => Ok(s),
            _ => Err(TextureAtlasError::Parse(format!("Invalid value type of '{}'", k))),
        }
    }

    fn to_u32(k: &str, v: &Value) -> Result<u32, TextureAtlasError> {
        match *v {
            Value::Integer(i) => {
                if i < 0 || i > ::std::u32::MAX as i64 {
                    Err(TextureAtlasError::Parse(format!("Invalid u32 value of '{}': {}", k, i)))
                } else {
                    Ok(i as u32)
                }
            },
            _ => Err(TextureAtlasError::Parse(format!("Invalid array value type of '{}'", k))),
        }
    }

    pub fn required_vec_of_u32(t: &Table, k: &str) -> Result<Vec<u32>, TextureAtlasError> {
        let val = try!(required_field(t, k));
        match *val {
            Value::Array(ref arr) => {
                Ok(arr.iter().map(|v| to_u32(k, v).unwrap()).collect())
            },
            _ => Err(TextureAtlasError::Parse(format!("Invalid value type of '{}'", k))),
        }
    }

    pub fn optional_vec_of_u32(t: &Table, k: &str) -> Result<Option<Vec<u32>>, TextureAtlasError> {
        let val = optional_field(t, k);
        match val {
            Some(&Value::Array(ref arr)) => {
                Ok(Some(arr.iter().map(|v| to_u32(k, v).unwrap()).collect()))
            },
            _ => Ok(None),
        }
    }

    pub fn vec_to_pair(k: &str, v: Vec<u32>) -> Result<(u32, u32), TextureAtlasError> {
        if v.len() != 2 {
            Err(TextureAtlasError::Parse(format!(
                "Invalid array length of '{}': {} (expected 2)", k, v.len())))
        } else {
            Ok((v[0], v[1]))
        }
    }

    pub fn vec_to_rbga8(k: &str, v: Vec<u32>) -> Result<image::Rgba<u8>, TextureAtlasError> {
        if v.len() != 4 {
            Err(TextureAtlasError::Parse(format!(
                "Invalid array length of '{}': {} (expected 4)", k, v.len())))
        } else {
            use std::u8;

            for i in &v {
                if *i > u8::MAX as u32 {
                    return Err(TextureAtlasError::Parse(format!(
                        "Invalid array value of '{}': {} (expected 0 <= x <= 255)", k, i)));
                }
            }

            Ok(image::Rgba { data: [v[0] as u8, v[1] as u8, v[2] as u8, v[3] as u8] })
        }
    }
}
