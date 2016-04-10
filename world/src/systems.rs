use fov;

use std::vec::Vec;

use super::WorldData;
use components;

pub trait System {
    fn update(&mut self, data: &mut WorldData);
}

pub struct LightingSystem;

impl System for LightingSystem {
    fn update(&mut self, data: &mut WorldData) {
        let mut result = Vec::new();

        for e in &data.entities {
            if let Some(g) = data.get_component::<components::Glow>(e) {
                if let Some(&components::Position { x, y, z }) = data.get_component::<components::Position>(e) {
                    // we got a omnidirectional light source
                    let r = g.radius;
                    let (lm_w, lm_h) = ((r * 2) + 1, (r * 2) + 1);
                    let lm_size = (lm_w * lm_h) as usize;

                    // TODO get rid of copying?
                    let mut g = components::Glow::new(r);
                    g.light_map_size = (lm_w, lm_h);
                    g.light_map = {
                        let mut v = Vec::with_capacity(lm_size);
                        v.resize(lm_size, 1.0);
                        v
                    };

                    for (dx, dy, o) in fov::iter(&data.map, x, y, z, r) {
                        let (gx, gy) = (dx + r as i32, dy + r as i32);
                        if gx < 0 || gx >= lm_w as i32 || gy < 0 || gy >= lm_h as i32 {
                            continue;
                        }
                        g.set_at(gx as u32, gy as u32, o);
                    }

                    result.push((*e, g));
                }
            }
        }

        for (e, g) in result {
            data.add_component(e, g);
        }
    }
}
