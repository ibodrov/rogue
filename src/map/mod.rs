use std::vec::Vec;

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

        Map {
            size: (width, height),
            data: data,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
