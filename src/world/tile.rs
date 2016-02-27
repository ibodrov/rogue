use std::slice;

#[derive(Debug)]
pub enum Effect {
    Lit(f32),
}

#[derive(Debug)]
pub struct Tile {
    pub ground: u8,
    pub effects: Option<Vec<Effect>>,
}

impl Tile {
    pub fn new(ground: u8) -> Self {
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

#[test]
fn test_iter() {
    use std::vec::Vec;

    let mut tiles = Vec::new();
    tiles.push(Tile::new(0));
    tiles.push(Tile::new(1));
    tiles.push(Tile::new(2));
    tiles.push(Tile::new(3));

    let mut it = TilesIter::new((2, 2, 2), tiles.iter());

    fn check(t: Option<(u32, u32, u32, &Tile)>, x: u32, y: u32, z: u32, g: u8) -> bool {
        if let Some((tx, ty, tz, &Tile { ground: tg, ..})) = t {
            tx == x && ty == y && tz == z && tg == g
        } else {
            false
        }
    };

    assert_eq!(check(it.next(), 0, 0, 0, 0), true);
    assert_eq!(check(it.next(), 1, 0, 0, 1), true);
    assert_eq!(check(it.next(), 0, 1, 0, 2), true);
    assert_eq!(check(it.next(), 1, 1, 0, 3), true);
}
