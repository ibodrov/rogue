use std::collections::HashMap;
use std::vec::Vec;
use std::fmt;

use world::EntityId;

pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct Glow {
    pub radius: u32,
    pub light_map_size: (u32, u32),
    pub light_map: Vec<f32>,
}

impl Glow {
    pub fn new(radius: u32) -> Self {
        Glow {
            radius: radius,
            light_map_size: (0, 0),
            light_map: Vec::new(),
        }
    }

    pub fn get_at(&self, x: u32, y: u32) -> f32 {
        debug_assert!(x < self.light_map_size.0 &&
                      y < self.light_map_size.1,
                      "Invalid light map point: {},{}", x, y);

        let idx = (x + y * self.light_map_size.0) as usize;
        self.light_map[idx]
    }

    pub fn set_at(&mut self, x: u32, y: u32, v: f32) {
        debug_assert!(x < self.light_map_size.0 &&
                      y < self.light_map_size.1,
                      "Invalid light map point: {},{}", x, y);

        let idx = (x + y * self.light_map_size.0) as usize;
        self.light_map[idx] = v;
    }
}

impl fmt::Debug for Glow {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for j in 0..self.light_map_size.1 {
            for i in 0..self.light_map_size.0 {
                write!(f, "{},", self.get_at(i, j)).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        Ok(())
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
