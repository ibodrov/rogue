use std::slice;

pub enum Effect {
    Lit(f32),
}

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
    x: u32,
    y: u32,
    width: u32,
    delegate: slice::Iter<'a, Tile>,
}

impl<'a> TilesIter<'a> {
    pub fn new(width: u32, delegate: slice::Iter<'a, Tile>) -> Self {
        TilesIter {
            x: 0,
            y: 0,
            width: width,
            delegate: delegate,
        }
    }
}

impl<'a> Iterator for TilesIter<'a> {
    /// (x, y, &tile)
    type Item = (u32, u32, &'a Tile);

    fn next(&mut self) -> Option<(u32, u32, &'a Tile)> {
        match self.delegate.next() {
            Some(t) => {
                let x = self.x;
                let y = self.y;

                self.x += 1;
                if self.x >= self.width {
                    self.x = 0;
                    self.y += 1;
                }

                Some((x, y, t))
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

    let mut it = TilesIter::new(2, tiles.iter());

    fn check(t: Option<(u32, u32, &Tile)>, x: u32, y: u32, g: u8) -> bool {
        if let Some((tx, ty, &Tile { ground: tg, ..})) = t {
            tx == x && ty == y && tg == g
        } else {
            false
        }
    };

    assert_eq!(check(it.next(), 0, 0, 0), true);
    assert_eq!(check(it.next(), 1, 0, 1), true);
    assert_eq!(check(it.next(), 0, 1, 2), true);
    assert_eq!(check(it.next(), 1, 1, 3), true);
}
