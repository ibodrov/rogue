use std::ops::{Index, IndexMut};
use chan;

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
}

impl Index<(u32, u32, u32)> for MapChunk {
    type Output = Cell;

    fn index<'a>(&'a self, coords: (u32, u32, u32)) -> &'a Cell {
        debug_assert!(self.is_inside(coords),
                      "Invalid coordinates. Size: {:?}, coordinates: {:?}", self.size, coords);

        let idx = self.idx(coords);
        &self.data[idx]
    }
}

impl IndexMut<(u32, u32, u32)> for MapChunk {
    fn index_mut<'a>(&'a mut self, coords: (u32, u32, u32)) -> &'a mut Cell {
        debug_assert!(self.is_inside(coords),
                      "Invalid coordinates. Size: {:?}, coordinates: {:?}", self.size, coords);

        let idx = self.idx(coords);
        &mut self.data[idx]
    }
}

#[derive(Clone)]
pub struct Map {
    chunk: MapChunk,
    sender: chan::Sender<MapChunk>,
    receiver: chan::Receiver<MapChunk>,
}

impl Map {
    pub fn new(size: (u32, u32, u32), v: Cell) -> Self {
        let (sender, receiver) = chan::async();

        Map {
            chunk: MapChunk::new((0, 0, 0), size, v),
            sender: sender,
            receiver: receiver,
        }
    }

    pub fn apply_updates(&mut self) {
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
                    }
                },
            }
        }
    }

    pub fn update(&mut self, chunk: MapChunk) {
        self.sender.send(chunk);
    }
}

impl Index<(u32, u32, u32)> for Map {
    type Output = Cell;

    fn index<'a>(&'a self, coords: (u32, u32, u32)) -> &'a Cell {
        &self.chunk[coords]
    }
}

impl IndexMut<(u32, u32, u32)> for Map {
    fn index_mut<'a>(&'a mut self, coords: (u32, u32, u32)) -> &'a mut Cell {
        &mut self.chunk[coords]
    }
}
