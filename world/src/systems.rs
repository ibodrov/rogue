extern crate rand;

use std::vec::Vec;
use std::cell::RefCell;
use std::collections::VecDeque;
use super::WorldData;
use self::rand::Rng;
use components;
use fov;

pub trait System {
    fn update(&mut self, data: &mut WorldData, time_dt: f64);
}

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, data: &mut WorldData, time_dt: f64) {
        let mut result = Vec::new();

        let (map_w, map_h, _) = data.map.size();

        for e in &data.entities {
            if let Some(ref s0) = data.components.get_component::<components::Speed>(e) {
                if let Some(ref p0) = data.components.get_component::<components::Position>(e) {
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
            data.components.add_component(e, speed);
            data.components.add_component(e, position);
        }
    }
}

pub struct LightingSystem;

impl System for LightingSystem {
    fn update(&mut self, data: &mut WorldData, _: f64) {
        let mut result = Vec::new();

        for e in &data.entities {
            if let Some(g) = data.components.get_component::<components::Glow>(e) {
                if let Some(&components::Position { x, y, z }) = data.components.get_component::<components::Position>(e) {
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
            data.components.add_component(e, g);
        }
    }
}

#[derive(Debug)]
pub enum KeyboardCommand {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct KeyboardControlSystem {
    queue: RefCell<VecDeque<KeyboardCommand>>,
}

impl Default for KeyboardControlSystem {
    fn default() -> Self {
        KeyboardControlSystem {
            queue: RefCell::new(VecDeque::new()),
        }
    }
}

impl KeyboardControlSystem {
    pub fn add(&mut self, cmd: KeyboardCommand) {
        self.queue.borrow_mut().push_back(cmd);
    }
}

impl System for KeyboardControlSystem {
    fn update(&mut self, data: &mut WorldData, _: f64) {
        let mut q = self.queue.borrow_mut();

        if q.is_empty() {
            return;
        }

        let cmd = q.pop_front().unwrap();
        println!("POP: {:?}", cmd);

        let mut result = Vec::new();

        for e in &data.entities {
            if let Some((_, &components::Position { x: x0, y: y0, z })) = data.components.join::<components::Controlled, components::Position>(e) {

                let mut x = x0;
                let mut y = y0;

                match cmd {
                    KeyboardCommand::UP => y -= 1,
                    KeyboardCommand::DOWN => y += 1,
                    KeyboardCommand::LEFT => x -= 1,
                    KeyboardCommand::RIGHT => x += 1,
                }

                let t = data.map.get_at(x, y, z);
                if *t == 1 {
                    continue;
                }

                result.push((*e, components::Position { x: x, y: y, z: z }));
            }
        }

        for (e, g) in result {
            data.components.add_component(e, g);
        }
    }
}
