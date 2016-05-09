#![feature(plugin)]
#![plugin(serde_macros)]
#![feature(custom_derive)]

extern crate serde;
extern crate serde_json;

#[derive(Debug)]
pub enum CfgError {
    IO(String),
    Parse(String),
}

fn load<T: serde::Deserialize>(path: &str) -> Result<T, CfgError> {
    use std::fs::File;
    use std::io::Read;

    let mut f = try!(File::open(path));
    let mut buf = String::new();
    try!(f.read_to_string(&mut buf));
    Ok(try!(serde_json::from_str(&buf)))
}

pub mod assets {
    #[derive(Serialize, Deserialize)]
    pub struct TextureAtlasCfg {
        pub path: String,
        pub tile_size: (u32, u32),
        pub tile_count: (u32, u32),
        pub color_mask: Option<[u8; 4]>,
    }

    pub fn load_atlas(path: &str) -> Result<TextureAtlasCfg, super::CfgError> {
        super::load(path)
    }
}

pub mod ui {
    use std::collections::HashMap;

    #[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct UiMapTileCfg {
        pub tile_n: u32,
        pub fg: [u8; 4],
        #[serde(default)]
        pub bg: [u8; 4],
    }

    impl Default for UiMapTileCfg {
        fn default() -> Self {
            UiMapTileCfg {
                tile_n: 0,
                fg: [255, 255, 255, 255],
                bg: [255, 255, 255, 255],
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct UiMapCfg {
        pub atlas_path: String,
        #[serde(skip_serializing_if="Option::is_none")]
        pub visible_tile_size: Option<(u32, u32)>,
        pub tiles: HashMap<String, UiMapTileCfg>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UiCfg {
        pub map: UiMapCfg,
    }

    pub fn load(path: &str) -> Result<UiCfg, super::CfgError> {
        super::load(path)
    }
}

impl From<serde_json::error::Error> for CfgError {
    fn from(e: serde_json::error::Error) -> Self {
        CfgError::Parse(format!("{:?}", e))
    }
}

impl From<std::io::Error> for CfgError {
    fn from(e: std::io::Error) -> Self {
        CfgError::IO(format!("{:?}", e))
    }
}
