extern crate jobsteal;

use std::sync::Arc;
use std::sync::Mutex;

use fov;
use super::WorldData;
use components;

pub trait System {
    fn update(&mut self, data: Arc<Mutex<WorldData>>);
}

pub struct LightingSystem {
    pool: jobsteal::Pool,
}

impl LightingSystem {
    pub fn new() -> Self {
        LightingSystem {
            pool: jobsteal::make_pool(4).unwrap(),
        }
    }
}

impl System for LightingSystem {
    fn update(&mut self, data: Arc<Mutex<WorldData>>) {
        self.pool.scope(|scope| {
            let entities = {
                data.lock().unwrap().entities.clone()
            };

            for e in entities {
                let handle = data.clone();

                scope.submit(move || {
                    let data = &mut *handle.lock().unwrap();
                    let mut cs = &mut data.components;

                    // TODO macro?
                    if let Some(ref mut g) = cs.glow.get_mut(&e) {
                        if let Some(&components::Position { x, y, z }) = cs.position.get(&e) {
                            // we got a omnidirectional light source
                            let r = g.radius;
                            let (lm_w, lm_h) = ((r * 2) + 1, (r * 2) + 1);
                            let lm_size = (lm_w * lm_h) as usize;

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
                        }
                    }
                });
            }
        });
    }
}
