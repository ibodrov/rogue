use std::collections::HashMap;
use std::vec::Vec;

use map;
use fov;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Luminocity(pub f32);

pub struct Components {
    pub position: HashMap<EntityId, Position>,
    pub luminocity: HashMap<EntityId, Luminocity>,
}

impl Components {
    fn new() -> Self {
        Components {
            position: HashMap::new(),
            luminocity: HashMap::new(),
        }
    }

}

#[derive(Debug)]
pub enum TileEffect {
    Lit(f32),
}

#[derive(Debug)]
pub struct Tile {
    pub ground: u8,
    pub effects: Option<Vec<TileEffect>>,
}

pub struct World {
    entities: Vec<EntityId>,
    components: Components,
    map: map::Map,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            entities: Vec::new(),
            components: Components::new(),
            map: map::Map::new(64, 64),
        };

        fn add_torch(w: &mut World, id: u64, x: u32, y: u32, lum: f32) {
            w.create_entity(EntityId(id), |id, cs| {
                cs.position.insert(id, Position { x: x, y: y });
                cs.luminocity.insert(id, Luminocity(lum));
            });
        };

        add_torch(&mut w, 0, 10, 10, 1.0);
        add_torch(&mut w, 1, 15, 15, 1.0);
        add_torch(&mut w, 2, 20, 20, 1.0);

        w
    }

    pub fn create_entity<F>(&mut self, id: EntityId, builder: F) where F: Fn(EntityId, &mut Components) {
        builder(id, &mut self.components);
        self.entities.push(id);
    }

    pub fn map(&self) -> &map::Map {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut map::Map {
        &mut self.map
    }

    pub fn update<F>(&mut self, f: F) where F: Fn(&mut Components) {
        f(&mut self.components);
    }

    pub fn render(&self) -> Vec<Tile> {
        let map = self.map();
        let (map_w, map_h) = map.size();
        let max_n = (map_w * map_h) as usize;
        let mut v = Vec::with_capacity(max_n);

        for j in 0..map_h {
            for i in 0..map_w {
                let g = self.map.get_at(i, j);
                let t = Tile { ground: g, effects: None };
                v.push(t);
            }
        }

        fn add_effect(t: &mut Tile, e: TileEffect) {
            if let None = t.effects {
                t.effects = Some(Vec::new());
            }

            if let Some(v) = t.effects.as_mut() {
                v.push(e);
            }
        }

        let cs = &self.components;
        for e in &self.entities {
            if let Some(&Position { x, y }) = cs.position.get(e) {
                if let Some(&Luminocity(lum)) = cs.luminocity.get(e) {
                    // we got a torch

                    fn fade(x1: u32, y1: u32, x2: u32, y2: u32, _r: u32) -> f32 {
                        let dt = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as f32;
                        1.0 / (1.0 + dt.sqrt())
                    }

                    let illum = |v: &mut Vec<Tile>, x: u32, y: u32, lum: f32| {
                        let n = (x + y * map_w) as usize;
                        let t = &mut v[n];
                        add_effect(t, TileEffect::Lit(lum));
                    };

                    illum(&mut v, x, y, lum);

                    let fov = fov::FOV::new(&map, x, y, 10);
                    for j in 0..map_h {
                        for i in 0..map_w {
                            if i == x && j == y {
                                continue;
                            }

                            let o = fov.get_at(i, j);
                            if o < 1.0 {
                                let coeff = fade(x, y, i, j, 10);
                                let lum = lum * (1.0 - o) * coeff;
                                illum(&mut v, i, j, lum);
                            }
                        }
                    }
                }
            }
        }

        v
    }
}

#[test]
fn test_world() {
    let w = World::new();
    let tiles = w.render();
    println!("{:?}", tiles);
}
