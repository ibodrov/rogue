extern crate rand;

use std::vec::Vec;
use self::rand::Rng;

pub struct Map {
    size: (u32, u32),
    data: Vec<u8>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let data: Vec<u8> = (0..width * height).map(|_| 0).collect();

        let mut m = Map {
            size: (width, height),
            data: data,
        };
        m.randomize();

        m
    }

    pub fn randomize(&mut self) {
        let (mw, mh) = self.size();
        let max_n = (mw * mh) as usize;
        let mut rng = rand::thread_rng();
        let idx = |x, y| { (x + y * mw) as usize };

        // clear
        for n in 0..self.data.len() {
            self.data[n] = 0;
        }

        // top and bottom wall
        for x in 0..mw {
            self.data[idx(x, 0)] = 1;
            self.data[idx(x, mh - 1)] = 1;
        }

        // left and right wall
        for y in 0..mh {
            self.data[idx(0, y)] = 1;
            self.data[idx(mw - 1, y)] = 1;
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
                    let n = (i + j * mw) as usize;
                    if n >= max_n {
                        continue;
                    }
                    self.data[n] = 1;
                }
            }
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn get_at(&self, x: u32, y: u32) -> u8 {
        let n = (x + y * self.size.0) as usize;
        self.data[n]
    }

    pub fn set_at(&mut self, x: u32, y: u32, t: u8) {
        let n = (x + y * self.size.0) as usize;
        self.data[n] = t;
    }
}
