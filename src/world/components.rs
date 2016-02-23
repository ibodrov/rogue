use std::collections::HashMap;

use world::EntityId;

pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Luminocity(pub f32);

pub struct Components {
    pub positions: HashMap<EntityId, Position>,
    pub luminocity: HashMap<EntityId, Luminocity>,
}

impl Components {
    pub fn new() -> Self {
        Components {
            positions: HashMap::new(),
            luminocity: HashMap::new(),
        }
    }
}
