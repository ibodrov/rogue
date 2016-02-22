extern crate rand;

use std::vec::Vec;
use rand::Rng;

pub struct Map {
    size: (u32, u32),
    data: Vec<u8>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let mut data: Vec<u8> = (0..width * height).map(|_| 0).collect();

        let idx = |x, y| { (x + y * width) as usize };

        // top and bottom wall
        for x in 0..width {
            data[idx(x, 0)] = 1;
            data[idx(x, height - 1)] = 1;
        }

        // left and right wall
        for y in 0..height {
            data[idx(0, y)] = 1;
            data[idx(width - 1, y)] = 1;
        }

        // random dots
        let mut rng = rand::thread_rng();
        for x in 1..width - 1 {
            for y in 1..height - 1 {
                let n = rng.gen_range(0, 100); // [0, 5)
                if n < 15 {
                    data[idx(x, y)] = 1;
                }
            }
        }

        Map {
            size: (width, height),
            data: data,
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
