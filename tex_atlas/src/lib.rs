extern crate glium;
extern crate image;
extern crate toml;

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

/*
pub fn _load<F: Facade>(display: &F, path: &Path) -> Result<TextureAtlas, TextureAtlasError> {
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
*/

pub fn load<F: Facade>(display: &F, path: &Path) -> Result<TextureAtlas, TextureAtlasError> {
    use std::fs::File;
    use std::io::Read;

    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    fn format_errors(errors: &Vec<toml::ParserError>) -> String {
        let mut s = String::new();
        for e in errors {
            s.push_str(&format!("{:?}", e));
            s.push_str("\n");
        }
        s
    }

    let mut p = toml::Parser::new(&s);
    match p.parse() {
        Some(table) => {
            use toml_utils;

            fn vec_to_pair(k: &str, v: Vec<u32>) -> Result<(u32, u32), TextureAtlasError> {
                if v.len() != 2 {
                    Err(TextureAtlasError::Parse(format!("Invalid array length of '{}': {}", k, v.len())))
                } else {
                    Ok((v[0], v[1]))
                }
            }

            let image_file = {
                let k = "file";
                try!(toml_utils::ensure_string(&table, k))
            };

            let tile_size = {
                let k = "tile_size";
                try!(vec_to_pair(k, try!(toml_utils::ensure_vec_of_u32(&table, k))))
            };

            let tile_count = {
                let k = "tile_count";
                try!(vec_to_pair(k, try!(toml_utils::ensure_vec_of_u32(&table, k))))
            };

            let img = try!(image::open(image_file)).to_rgba();
            let dimensions = img.dimensions();

            let img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.into_raw(), dimensions);
            let tex = try!(Texture2d::new(display, img));

            Ok(TextureAtlas {
                texture: tex,
                tile_size: tile_size,
                tile_count: tile_count,
            })
        },
        _ => Err(TextureAtlasError::Parse(format_errors(&p.errors))),
    }
}

mod toml_utils {
    use toml::{Table, Value};
    use super::TextureAtlasError;

    fn ensure_field<'a>(t: &'a Table, k: &str) -> Result<&'a Value, TextureAtlasError> {
        match t.get(k) {
            Some(v) => Ok(v),
            _ => Err(TextureAtlasError::Parse(format!("Missing value of '{}'", k))),
        }
    }

    pub fn ensure_string<'a>(t: &'a Table, k: &str) -> Result<&'a String, TextureAtlasError> {
        let v = try!(ensure_field(t, k));
        match v {
            &Value::String(ref s) => Ok(s),
            _ => Err(TextureAtlasError::Parse(format!("Invalid value type of '{}'", k))),
        }
    }

    fn to_u32(k: &str, v: &Value) -> Result<u32, TextureAtlasError> {
        match v {
            &Value::Integer(i) => {
                if i < 0 || i > ::std::u32::MAX as i64 {
                    Err(TextureAtlasError::Parse(format!("Invalid u32 value of '{}': {}", k, i)))
                } else {
                    Ok(i as u32)
                }
            },
            _ => Err(TextureAtlasError::Parse(format!("Invalid array value type of '{}'", k))),
        }
    }

    pub fn ensure_vec_of_u32<'a>(t: &'a Table, k: &str) -> Result<Vec<u32>, TextureAtlasError> {
        let v = try!(ensure_field(t, k));
        match v {
            &Value::Array(ref arr) => {
                Ok(arr.iter().map(|v| to_u32(k, v).unwrap()).collect())
            },
            _ => Err(TextureAtlasError::Parse(format!("Invalid value type of '{}'", k))),
        }
    }
}
