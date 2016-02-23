pub mod map;
pub mod components;
pub mod tile;
pub mod render;
mod systems;

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

pub struct WorldData {
    pub map: map::Map<u8>,
    pub entities: Vec<EntityId>,
    pub components: components::Components,
}

pub struct World {
    data: WorldData,
    systems: Vec<Box<systems::System>>,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            data: WorldData {
                map: map::Map::new(128, 128, 0),
                entities: Vec::new(),
                components: components::Components::new(),
            },
            systems: vec![Box::new(systems::LightingSystem)],
        };

        fn add_torch(w: &mut World, id: u64, x: u32, y: u32, radius: u32) {
            w.create_entity(EntityId(id), |id, cs| {
                cs.position.insert(id, components::Position { x: x, y: y });
                cs.glow.insert(id, components::Glow::new(radius));
            });
        };

        add_torch(&mut w, 0, 10, 10, 10);
        add_torch(&mut w, 1, 15, 15, 10);
        add_torch(&mut w, 2, 50, 50, 10);
        add_torch(&mut w, 3, 85, 64, 10);

        w
    }

    pub fn data(&self) -> &WorldData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut WorldData {
        &mut self.data
    }

    pub fn create_entity<F>(&mut self, id: EntityId, builder: F)
        where F: Fn(EntityId, &mut components::Components) {

        builder(id, &mut self.data_mut().components);
        self.data.entities.push(id);
    }

    pub fn update<F>(&mut self, f: F) where F: Fn(&mut components::Components) {
        f(&mut self.data_mut().components);
    }

    pub fn tick(&mut self) {
        for s in self.systems.iter_mut() {
            s.update(&mut self.data);
        }
    }
}
