extern crate time;

pub mod map;
pub mod components;
pub mod tile;
pub mod render;
mod fov;
mod circle_iter;
mod systems;

use std::vec::Vec;
use std::collections::HashMap;
use std::any::{Any, TypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

pub struct WorldData {
    pub map: map::Map<u8>,
    pub entities: Vec<EntityId>,
    components: HashMap<TypeId, HashMap<EntityId, Box<Any>>>,
}

impl WorldData {
    fn ensure_components(&mut self, t: TypeId) -> &mut HashMap<EntityId, Box<Any>> {
        let m = &mut self.components;

        if m.contains_key(&t) {
            return m.get_mut(&t).unwrap();
        }

        let c = HashMap::new();
        m.insert(t, c);

        m.get_mut(&t).unwrap()
    }

    fn add_component<T: Any>(&mut self, id: EntityId, c: T) {
        let t = TypeId::of::<T>();
        let cs = self.ensure_components(t);
        cs.insert(id, Box::new(c));
    }

    pub fn get_component<T: Any>(&self, id: &EntityId) -> Option<&T> {
        let t = TypeId::of::<T>();
        if let Some(cs) = self.components.get(&t) {
            match cs.get(id) {
                Some(c) => Some(c.downcast_ref::<T>().unwrap()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_component_mut<T: Any>(&mut self, id: &EntityId) -> Option<&mut T> {
        let t = TypeId::of::<T>();
        if let Some(cs) = self.components.get_mut(&t) {
            match cs.get_mut(id) {
                Some(c) => Some(c.downcast_mut::<T>().unwrap()),
                _ => None,
            }
        } else {
            None
        }
    }
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
                components: HashMap::new(),
            },
            systems: vec![Box::new(systems::LightingSystem),
                          Box::new(systems::MovementSystem)],
            last_timestamp: time::precise_time_s()
        };

        /*
        add_torch(&mut w, 10, 10, 10);
         */
        w.data.map.randomize(1, 0);

        for x in 1..11 {
            add_bouncer(&mut w, x * 5, x * 5);
        }

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

        for (_, v) in self.data.components.iter_mut() {
            v.remove(&id);
        }
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
        data.add_component(id, components::Position { x: x, y: y, z: 0 });
        data.add_component(id, components::Glow::new(radius));
    });
}

pub fn add_bouncer(w: &mut World, x: u32, y: u32) {
    let id = w.create_entity_id();
    w.create_entity(id, |id, data| {
        data.add_component(id, components::Position { x: x, y: y, z: 0});
        data.add_component(id, components::Speed::rand());
        //data.add_component(id, components::Visible::default());
        data.add_component(id, components::Glow::new(10));
    });
}
