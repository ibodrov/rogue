use std::ops::{Index, IndexMut};

pub type Cell = u8;

#[derive(Clone)]
pub struct Map {
    size: (u32, u32, u32),
    data: Vec<Cell>,
}

impl Index<(u32, u32, u32)> for Map {
    type Output = Cell;

    fn index(&self, coords: (u32, u32, u32)) -> &Cell {
        debug_assert!(self.is_inside(coords),
                      "Invalid coordinates. Size: {:?}, coordinates: {:?}", self.size, coords);

        let idx = self.idx(coords);
        &self.data[idx]
    }
}

impl IndexMut<(u32, u32, u32)> for Map {
    fn index_mut(&mut self, coords: (u32, u32, u32)) -> &mut Cell {
        debug_assert!(self.is_inside(coords),
                      "Invalid coordinates. Size: {:?}, coordinates: {:?}", self.size, coords);

        let idx = self.idx(coords);
        &mut self.data[idx]
    }
}

impl Map {
    pub fn new(size: (u32, u32, u32), v: Cell) -> Self {
        Map {
            size: size,
            data: (0..size.0 * size.1 * size.2).map(|_| v).collect(),
        }
    }

    fn is_inside(&self, coords: (u32, u32, u32)) -> bool {
        let (sx, sy, sz) = self.size;
        coords.0 < sx && coords.1 < sy && coords.2 < sz
    }

    fn idx(&self, coords: (u32, u32, u32)) -> usize {
        let (sx, sy, _) = self.size;
        (coords.0 + coords.1 * sx + coords.2 * sx * sy) as usize
    }

    pub fn fill(&mut self, v: Cell) {
        for i in &mut self.data {
            *i = v;
        }
    }

    pub fn size(&self) -> (u32, u32, u32) {
        self.size
    }
}
