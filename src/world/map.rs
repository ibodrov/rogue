extern crate rand;

use std::vec::Vec;
use self::rand::Rng;

pub struct Map<T: Clone + Copy> {
    size: (u32, u32, u32),
    data: Vec<T>,
}

impl<T: Clone + Copy> Map<T> {
    pub fn new(width: u32, height: u32, v: T) -> Self {
        let levels = 1;

        Map {
            size: (width, height, levels),
            data: (0..width*height*levels).map(|_| v).collect(),
        }
    }

    fn idx(&self, x: u32, y: u32, level: u32) -> usize {
        let (dx, dy, _) = self.size;
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
        let (mw, mh, _) = self.size();
        let mut rng = rand::thread_rng();

        // clear
        self.fill(nothing);

        // top and bottom wall
        for x in 0..mw {
            self.set_at(x, 0, 0, wall);
            self.set_at(x, mh - 1, 0, wall);
        }

        // left and right wall
        for y in 0..mh {
            self.set_at(0, y, 0, wall);
            self.set_at(mw - 1, y, 0, wall);
        }

        // random boxes
        let cnt = 100;
        for _ in 0..cnt {
            let x = rng.gen_range(1, mw - 1);
            let y = rng.gen_range(1, mh - 1);
            let w = rng.gen_range(2, 5);
            let h = rng.gen_range(2, 5);

            for j in y..y+h {
                for i in x..x+w {
                    if i >= mw || j >= mh {
                        continue;
                    }

                    self.set_at(i, j, 0, wall);
                }
            }
        }

    }
}
