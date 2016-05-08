use std::ops::{Index, IndexMut};

pub type Cell = u8;

#[derive(Clone)]
pub struct MapChunk {
    position: (u32, u32, u32),
    size: (u32, u32, u32),
    data: Vec<Cell>,
}

impl MapChunk {
    pub fn new(position: (u32, u32, u32), size: (u32, u32, u32), v: Cell) -> Self {
        MapChunk {
            position: position,
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
        for i in self.data.iter_mut() {
            *i = v;
        }
    }
}

impl Index<(u32, u32, u32)> for MapChunk {
    type Output = Cell;

    fn index(&self, coords: (u32, u32, u32)) -> &Cell {
        debug_assert!(self.is_inside(coords),
                      "Invalid coordinates. Size: {:?}, coordinates: {:?}", self.size, coords);

        let idx = self.idx(coords);
        &self.data[idx]
    }
}

impl IndexMut<(u32, u32, u32)> for MapChunk {
    fn index_mut(&mut self, coords: (u32, u32, u32)) -> &mut Cell {
        debug_assert!(self.is_inside(coords),
                      "Invalid coordinates. Size: {:?}, coordinates: {:?}", self.size, coords);

        let idx = self.idx(coords);
        &mut self.data[idx]
    }
}

#[derive(Clone)]
pub struct Map {
    chunk: MapChunk,
}

impl Map {
    pub fn new(size: (u32, u32, u32), v: Cell) -> Self {
        Map {
            chunk: MapChunk::new((0, 0, 0), size, v),
        }
    }

    pub fn apply_updates(&mut self) {
        /*
        let dst = &mut self.chunk;
        let recv = &mut self.receiver;

        loop {
            chan_select! {
                default => {
                    break;
                },

                recv.recv() -> val => {
                    if let Some(src) = val {
                        let (px, py, pz) = src.position;
                        let (sx, sy, sz) = src.size;

                        // TODO optimize
                        for z in pz..pz + sz {
                            for y in py..py + sy {
                                for x in px..px + sx {
                                    dst[(x, y, z)] = src[(x - px, y - py, z - pz)];
                                }
                            }
                        }

                        debug!("Changes applied: {:?}, {:?}", src.position, src.size);
                    }
                },
            }
        }
        */
    }

    pub fn update(&mut self, chunk: MapChunk) {
        debug!("Sending changes: {:?}, {:?}", chunk.size, chunk.position);
        //self.sender.send(chunk);
    }

    pub fn size(&self) -> (u32, u32, u32) {
        self.chunk.size
    }
}

impl Index<(u32, u32, u32)> for Map {
    type Output = Cell;

    fn index(&self, coords: (u32, u32, u32)) -> &Cell {
        &self.chunk[coords]
    }
}

impl IndexMut<(u32, u32, u32)> for Map {
    fn index_mut(&mut self, coords: (u32, u32, u32)) -> &mut Cell {
        &mut self.chunk[coords]
    }
}
