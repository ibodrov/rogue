extern crate toml;

use std::collections::HashMap;
use std::path::Path;
use tex_atlas;
use glium::backend::Facade;

#[derive(Eq, PartialEq, Hash)]
pub struct MapEntityCfg {
    tile: u32,
    fg: [u8; 4],
}

impl MapEntityCfg {
    pub fn tile(&self) -> u32 {
        self.tile
    }

    pub fn fg_color(&self) -> [u8; 4] {
        self.fg
    }
}

pub struct MapCfg {
    atlas: tex_atlas::TextureAtlas,
    visible_tile_size: (u32, u32),
    entities: HashMap<String, MapEntityCfg>,
}

impl MapCfg {
    pub fn atlas(&self) -> &tex_atlas::TextureAtlas {
        &self.atlas
    }

    pub fn visible_tile_size(&self) -> (u32, u32) {
        self.visible_tile_size
    }

    pub fn entities(&self) -> &HashMap<String, MapEntityCfg> {
        &self.entities
    }
}

pub struct Configuration {
    map_cfg: MapCfg,
}

impl Configuration {
    pub fn map_cfg(&self) -> &MapCfg {
        &self.map_cfg
    }
}

// TODO rewrite
pub fn load<F: Facade>(display: &F, path: &str) -> Configuration {
    use std::fs::File;
    use std::io::Read;
    use self::toml::Value;

    // TODO error handling
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut p = toml::Parser::new(&s);
    match p.parse() {
        Some(table) => {
            match table.get("map") {
                Some(&Value::Table(ref table)) => {
                    let f = table.get("atlas").unwrap().as_str().unwrap();
                    let atlas = tex_atlas::load(display, Path::new(f)).unwrap();

                    let visible_tile_size = {
                        let s = table.get("visible_tile_size").unwrap().as_slice().unwrap();
                        (s[0].as_integer().unwrap() as u32, s[1].as_integer().unwrap() as u32)
                    };
                    debug!("visible_tile_size: {:?}", visible_tile_size);

                    let mut entities = HashMap::new();
                    match table.get("tiles") {
                        Some(&Value::Table(ref table)) => {
                            for (k, e) in table {
                                match e {
                                    &Value::Table(ref table) => {
                                        let tile = table.get("tile").unwrap().as_integer().unwrap() as u32;
                                        let fg = table.get("fg").unwrap().as_slice().unwrap().iter().map(|v| v.as_integer().unwrap() as u8).collect::<Vec<_>>();
                                        debug!("map.tiles: tile={:?}, fg={:?}", tile, fg);

                                        let k = k.clone();
                                        let v = MapEntityCfg {
                                            tile: tile,
                                            fg: [fg[0], fg[1], fg[2], fg[3]],
                                        };

                                        entities.insert(k, v);
                                    },
                                    _ => panic!("Error while parsing UI declarations #4"),
                                }
                            }
                        },
                        _ => panic!("Error while parsing UI declarations #3"),
                    }

                    let vis = Configuration {
                        map_cfg: MapCfg {
                            atlas: atlas,
                            visible_tile_size: visible_tile_size,
                            entities: entities,
                        },
                    };

                    vis
                },
                _ => panic!("Error while parsing UI declarations #2"),
            }
        },
        _ => panic!("Error while parsing UI declarations #1"),
    }
}
