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
    next_entity_id: u64,
    data: WorldData,
    systems: Vec<Box<systems::System>>,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            next_entity_id: 0,
            data: WorldData {
                map: map::Map::new(128, 128, 0),
                entities: Vec::new(),
                components: components::Components::new(),
            },
            systems: vec![Box::new(systems::LightingSystem)],
        };

        add_torch(&mut w, 10, 10, 10);

        w
    }

    pub fn create_entity_id(&mut self) -> EntityId {
        let eid = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        eid
    }

    pub fn delete_entity(&mut self, idx: usize) {
        let e = &mut self.data.entities;
        if e.len() == 0 {
            return;
        }

        let id = e.remove(idx);

        let c = &mut self.data.components;
        c.position.remove(&id);
        c.glow.remove(&id);
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

pub fn add_torch(w: &mut World, x: u32, y: u32, radius: u32) {
    let id = w.create_entity_id();
    w.create_entity(id, |id, cs| {
        cs.position.insert(id, components::Position { x: x, y: y });
        cs.glow.insert(id, components::Glow::new(radius));
    });
}
