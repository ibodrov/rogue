use std::collections::HashMap;
use std::any::{Any, TypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

pub struct Data {
    components: HashMap<TypeId, HashMap<EntityId, Box<Any>>>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            components: HashMap::new(),
        }
    }

    fn ensure_components(&mut self, t: TypeId) -> &mut HashMap<EntityId, Box<Any>> {
        let m = &mut self.components;

        if m.contains_key(&t) {
            return m.get_mut(&t).unwrap();
        }

        let c = HashMap::new();
        m.insert(t, c);

        m.get_mut(&t).unwrap()
    }

    pub fn add_component<T: Any>(&mut self, id: EntityId, c: T) {
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

    pub fn delete_entity(&mut self, id: &EntityId) {
        for (_, v) in self.components.iter_mut() {
            v.remove(id);
        }
    }
}
