extern crate rand;

use fov;

use std::vec::Vec;

use super::WorldData;
use components;

use self::rand::Rng;

pub trait System {
    fn update(&mut self, data: &mut WorldData, time_dt: f64);
}

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, data: &mut WorldData, time_dt: f64) {
        let mut result = Vec::new();

        let (map_w, map_h, _) = data.map.size();

        for e in &data.entities {
            if let Some(ref s0) = data.get_component::<components::Speed>(e) {
                if let Some(ref p0) = data.get_component::<components::Position>(e) {
                    let mut tile_x = p0.x as i32;
                    let mut tile_y = p0.y as i32;

                    let mut dx = s0.dx;
                    let mut dy = s0.dy;

                    let mut fract_x = s0.x + (dx as f32 * time_dt as f32);
                    let mut fract_y = s0.y + (dy as f32 * time_dt as f32);

                    if fract_x.abs() >= 1.0 {
                        tile_x += fract_x.signum() as i32;
                        fract_x = 0.0;
                    }

                    if fract_y.abs() >= 1.0 {
                        tile_y += fract_y.signum() as i32;
                        fract_y = 0.0;
                    }

                    if tile_x < 0 {
                        tile_x = 0;
                        dx *= -1;
                    }

                    if tile_y < 0 {
                        tile_y = 0;
                        dx *= -1;
                    }

                    if tile_x >= map_w as i32 {
                        tile_x = map_w as i32 - 1;
                        dx *= -1;
                    }

                    if tile_y >= map_h as i32 {
                        tile_y = map_h as i32 - 1;
                        dy *= -1;
                    }

                    let mut tile_x = tile_x as u32;
                    let mut tile_y = tile_y as u32;

                    let t = data.map.get_at(tile_x, tile_y, p0.z);
                    if *t == 1 {
                        tile_x = p0.x;
                        tile_y = p0.y;

                        let mut rng = rand::thread_rng();
                        let signs = [1, -1];

                        dx *= *rng.choose(&signs).unwrap();
                        dy *= *rng.choose(&signs).unwrap();
                    }

                    result.push((*e,
                                 components::Speed { x: fract_x, y: fract_y, dx: dx, dy: dy },
                                 components::Position { x: tile_x, y: tile_y, z: p0.z }));
                }
            }
        }

        for (e, speed, position) in result {
            data.add_component(e, speed);
            data.add_component(e, position);
        }
    }
}

pub struct LightingSystem;

impl System for LightingSystem {
    fn update(&mut self, data: &mut WorldData, _: f64) {
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
