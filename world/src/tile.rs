use std::slice;
use super::map;

#[derive(Debug)]
pub enum Effect {
    Lit(f32),
    Marked(u8),
}

#[derive(Debug)]
pub struct Tile {
    pub ground: map::Cell,
    pub effects: Option<Vec<Effect>>,
}

impl Tile {
    pub fn new(ground: map::Cell) -> Self {
        Tile {
            ground: ground,
            effects: None,
        }
    }

    pub fn add_effect(&mut self, e: Effect) {
        if let None = self.effects {
            self.effects = Some(Vec::new());
        }

        if let Some(ref mut v) = self.effects {
            v.push(e);
        }
    }
}

pub struct TilesIter<'a> {
    position: (u32, u32, u32),
    size: (u32, u32, u32),
    delegate: slice::Iter<'a, Tile>,
}

impl<'a> TilesIter<'a> {
    pub fn new(size: (u32, u32, u32), delegate: slice::Iter<'a, Tile>) -> Self {
        TilesIter {
            position: (0, 0, 0),
            size: size,
            delegate: delegate,
        }
    }
}

impl<'a> Iterator for TilesIter<'a> {
    /// (x, y, z, &tile)
    type Item = (u32, u32, u32, &'a Tile);

    fn next(&mut self) -> Option<(u32, u32, u32, &'a Tile)> {
        match self.delegate.next() {
            Some(t) => {
                let (ref mut x, ref mut y, ref mut z) = self.position;
                let (w, h, _) = self.size;

                let result = Some((*x, *y, *z, t));

                *x += 1;
                if *x >= w {
                    *x = 0;
                    *y += 1;
                    if *y >= h {
                        *y = 0;
                        *z += 1;
                    }
                }

                result
            },
            None => None,
        }
    }
}
