pub mod map;
pub mod components;
pub mod tile;
pub mod render;

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

pub struct World {
    map: map::Map<u8>,
    entities: Vec<EntityId>,
    components: components::Components,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            map: map::Map::new(128, 128, 0),
            entities: Vec::new(),
            components: components::Components::new(),
        };

        fn add_torch(w: &mut World, id: u64, x: u32, y: u32, lum: f32) {
            w.create_entity(EntityId(id), |id, cs| {
                cs.position.insert(id, components::Position { x: x, y: y });
                cs.glow.insert(id, components::Glow::new(lum));
            });
        };

        add_torch(&mut w, 0, 10, 10, 1.0);
        add_torch(&mut w, 1, 15, 15, 1.0);
        add_torch(&mut w, 2, 50, 50, 1.0);
        add_torch(&mut w, 3, 85, 64, 1.0);

        w
    }

    pub fn map(&self) -> &map::Map<u8> {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut map::Map<u8> {
        &mut self.map
    }

    pub fn create_entity<F>(&mut self, id: EntityId, builder: F)
        where F: Fn(EntityId, &mut components::Components) {

        builder(id, &mut self.components);
        self.entities.push(id);
    }

    pub fn update<F>(&mut self, f: F) where F: Fn(&mut components::Components) {
        f(&mut self.components);
    }
}
