extern crate rand;

use std::vec::Vec;
use self::rand::Rng;

pub struct Map<T: Clone + Copy> {
    // dimensions of the cells vector (x, y, z)
    size: (u32, u32, u32),
    data: Vec<T>,
}

impl<T: Clone + Copy> Map<T> {
    pub fn new(x_size: u32, y_size: u32, z_size: u32, v: T) -> Self {
        Map {
            size: (x_size, y_size, z_size),
            data: (0..x_size * y_size * z_size).map(|_| v).collect(),
        }
    }

    pub fn is_inside(&self, x: u32, y: u32, level: u32) -> bool {
        let (dx, dy, dz) = self.size;
        x < dx && y < dy && level < dz
    }

    fn idx(&self, x: u32, y: u32, level: u32) -> usize {
        let (dx, dy, _) = self.size;
        debug_assert!(self.is_inside(x, y, level), "Invalid map coordinates. Size: {:?}, coordinates: {}, {}, {}", self.size, x, y, level);
        (x + y * dx + level * dx * dy) as usize
    }

    pub fn get_at(&self, x: u32, y: u32, level: u32) -> &T {
        &self.data[self.idx(x, y, level)]
    }

    fn set_at(&mut self, x: u32, y: u32, level: u32, v: T) {
        let idx = self.idx(x, y, level);
        self.data[idx] = v;
    }

    fn fill(&mut self, v: T) {
        for i in self.data.iter_mut() {
            *i = v;
        }
    }

    pub fn size(&self) -> (u32, u32, u32) {
        self.size
    }

    pub fn randomize(&mut self, wall: T, nothing: T) {
        let (mx, my, mz) = self.size();
        let mut rng = rand::thread_rng();

        // clear
        self.fill(nothing);

        for z in 0..mz {
            // top and bottom wall
            for x in 0..mx {
                self.set_at(x, 0, z, wall);
                self.set_at(x, my - 1, z, wall);
            }

            // left and right wall
            for y in 0..my {
                self.set_at(0, y, z, wall);
                self.set_at(mx - 1, y, z, wall);
            }

            // random boxes
            let cnt = 100;
            for _ in 0..cnt {
                let x = rng.gen_range(1, mx - 1);
                let y = rng.gen_range(1, my - 1);
                let w = rng.gen_range(2, 5);
                let h = rng.gen_range(2, 5);

                for j in y..y+h {
                    for i in x..x+w {
                        if i >= mx || j >= my {
                            continue;
                        }

                        self.set_at(i, j, z, wall);
                    }
                }
            }
        }
    }
}
