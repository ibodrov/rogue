use world::WorldData;
use world::components;

pub trait System {
    fn update(&mut self, data: &mut WorldData);
}

pub struct LightingSystem;

impl System for LightingSystem {
    fn update(&mut self, data: &mut WorldData) {
        let cs = &mut data.components;
        for e in &data.entities {
            // TODO macro?
            if let Some(&mut components::Glow { ref radius,
                                                ref mut light_map_size,
                                                ref mut light_map }) = cs.glow.get_mut(e) {

                println!("{}, {:?}, {:?}", radius, light_map_size, light_map);
            }
        }
    }
}
