pub mod map;
pub mod tile;
pub mod render;

pub struct EntityId(pub u64);

pub struct World {
    map: map::Map<u8>,
}

impl World {
    pub fn new() -> Self {
        World {
            map: map::Map::new(128, 128, 0),
        }
    }

    pub fn map(&self) -> &map::Map<u8> {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut map::Map<u8> {
        &mut self.map
    }
}
