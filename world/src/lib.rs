extern crate time;
extern crate ecs;

pub mod map;
pub mod components;
pub mod tile;
pub mod render;
mod fov;
mod circle_iter;
mod systems;

use std::vec::Vec;
use ecs::EntityId;

pub struct WorldData {
    pub map: map::Map<u8>,
    pub entities: Vec<EntityId>,
    pub components: ecs::Data,
}

pub struct World {
    next_entity_id: u64,
    data: WorldData,
    systems: Vec<Box<systems::System>>,
    last_timestamp: f64,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            next_entity_id: 0,
            data: WorldData {
                map: map::Map::new(128, 128, 0),
                entities: Vec::new(),
                components: ecs::Data::new(),
            },
            systems: vec![Box::new(systems::LightingSystem),
                          Box::new(systems::MovementSystem),
                          Box::new(systems::KeyboardControlSystem::default())],
            last_timestamp: time::precise_time_s()
        };

        /*
        add_torch(&mut w, 10, 10, 10);
         */
        w.data.map.randomize(1, 0);

        /*
        for x in 1..11 {
            add_bouncer(&mut w, x * 5, x * 5);
        }*/

        add_dwarf(&mut w, 10, 10);

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
        self.data.components.delete_entity(&id);
    }

    pub fn data(&self) -> &WorldData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut WorldData {
        &mut self.data
    }

    pub fn create_entity<F>(&mut self, id: EntityId, builder: F)
        where F: Fn(EntityId, &mut WorldData) {

        builder(id, self.data_mut());
        self.data.entities.push(id);
    }

    pub fn update<F>(&mut self, f: F) where F: Fn(&mut WorldData) {
        f(self.data_mut());
    }

    pub fn tick(&mut self) {
        let current_timestamp = time::precise_time_s();
        let dt = self.last_timestamp - current_timestamp;
        self.last_timestamp = current_timestamp;

        for s in self.systems.iter_mut() {
            s.update(&mut self.data, dt);
        }
    }
}

pub fn add_torch(w: &mut World, x: u32, y: u32, radius: u32) {
    let id = w.create_entity_id();
    w.create_entity(id, |id, data| {
        data.components.add_component(id, components::Position { x: x, y: y, z: 0 });
        data.components.add_component(id, components::Glow::new(radius));
    });
}

pub fn add_bouncer(w: &mut World, x: u32, y: u32) {
    let id = w.create_entity_id();
    w.create_entity(id, |id, data| {
        data.components.add_component(id, components::Position { x: x, y: y, z: 0});
        data.components.add_component(id, components::Speed::rand());
        //data.add_component(id, components::Visible::default());
        data.components.add_component(id, components::Glow::new(10));
    });
}

pub fn add_dwarf(w: &mut World, x: u32, y: u32) {
    let id = w.create_entity_id();
    w.create_entity(id, |id, data| {
        data.components.add_component(id, components::Visible::default());
        data.components.add_component(id, components::Position { x: x, y: y, z: 0});
    });
}
