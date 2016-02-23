use std::collections::HashMap;
use std::vec::Vec;

use world::EntityId;

pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Glow {
    pub radius: u32,
    pub light_map_size: (u32, u32),
    pub light_map: Vec<(i32, i32, f32)>,
}

impl Glow {
    pub fn new(radius: u32) -> Self {
        Glow {
            radius: radius,
            light_map_size: (0, 0),
            light_map: Vec::new(),
        }
    }
}

pub struct Components {
    pub position: HashMap<EntityId, Position>,
    pub glow: HashMap<EntityId, Glow>,
}

impl Components {
    pub fn new() -> Self {
        Components {
            position: HashMap::new(),
            glow: HashMap::new(),
        }
    }
}
