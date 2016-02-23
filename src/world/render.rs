use std::vec::Vec;
use std::cmp;

use fov;
use world;
use world::tile;
use world::components;

// The "window" to the map, described in absolute coordinates.
#[derive(Debug, Clone, Copy)]
pub struct View {
    x: i32,
    y: i32,
    level: u32,
    width: u32,
    height: u32,
}

impl View {
    pub fn new(x: i32, y: i32, level: u32, width: u32, height: u32) -> Self {
        View {
            x: x,
            y: y,
            level: level,
            width: width,
            height: height,
        }
    }

    fn normalize(&self) -> NormalizedView {
        let x = cmp::max(self.x, 0) as u32;
        let y = cmp::max(self.y, 0) as u32;
        let w = self.width - self.x.abs() as u32;
        let h = self.height - self.y.abs() as u32;
        NormalizedView::new(x, y, self.level, w, h)
    }
}

// Normalized "window" to the map.
#[derive(Debug, Clone, Copy)]
pub struct NormalizedView {
    x: u32,
    y: u32,
    level: u32,
    width: u32,
    height: u32,
}

impl NormalizedView {
    fn new(x: u32, y: u32, level: u32, width: u32, height: u32) -> Self {
        NormalizedView {
            x: x,
            y: y,
            level: level,
            width: width,
            height: height,
        }
    }
}

pub struct RenderedWorldView {
    normalized_view: NormalizedView,
    tiles: Vec<tile::Tile>,
}

impl RenderedWorldView {
    pub fn tiles_count(&self) -> u32 {
        self.tiles.len() as u32
    }

    pub fn iter(&self) -> tile::TilesIter {
        let v = &self.normalized_view;
        tile::TilesIter::new(v.width, self.tiles.iter())
    }

    pub fn get_abs_mut<'a>(tiles: &'a mut Vec<tile::Tile>, v: &NormalizedView, x: u32, y: u32, level: u32) -> Option<&'a mut tile::Tile> {
        if x < v.x || x >= v.x + v.width || y < v.y || y >= v.y + v.height {
            return None;
        }

        let (nx, ny) = (x - v.x, y - v.y);
        let n = (nx + ny * v.width + level * v.width * v.height) as usize;

        Some(&mut tiles[n])
    }
}

pub trait Renderable {
    fn render(&self, view: &View) -> RenderedWorldView;
}

impl Renderable for world::World {
    fn render(&self, view: &View) -> RenderedWorldView {
        // get the actual (normalized) size of the view
        let n_view = view.normalize();

        let (view_w, view_h, view_level) = (n_view.width,
                                            n_view.height,
                                            n_view.level);

        // estimate the count of the tiles
        let max_tiles = (view_w * view_h) as usize;
        let mut tiles = Vec::with_capacity(max_tiles);

        // determine the range of tiles
        let map_start_i = cmp::max(view.x, 0) as u32;
        let map_end_i = map_start_i + view_w;
        let map_start_j = cmp::max(view.y, 0) as u32;
        let map_end_j = map_start_j + view_h;

        // convert map's data into tiles
        for j in map_start_j..map_end_j {
            for i in map_start_i..map_end_i {
                let g = self.map.get_at(i, j, view_level);
                let t = tile::Tile::new(*g);
                tiles.push(t);
            }
        }

        let (map_w, map_h, _) = self.map.size();

        // render entities
        let cs = &self.components;
        for e in &self.entities {
            if let Some(&components::Position { x, y }) = cs.position.get(e) {
                if let Some(&components::Glow { strength, .. }) = cs.glow.get(e) {
                    // we got a torch

                    // TODO radius?
                    let radius = 10;

                    // TODO check if the torch or his glow are visible

                    let illum = |ts: &mut Vec<tile::Tile>, v: &NormalizedView, x: u32, y: u32, lum: f32| {
                        // TODO level?
                        if let Some(t) = RenderedWorldView::get_abs_mut(ts, v, x, y, 0) {
                            t.add_effect(tile::Effect::Lit(lum));
                        }
                    };

                    illum(&mut tiles, &n_view, x, y, strength);

                    let fov = fov::FOV::new(&self.map, x, y, 0, radius);
                    for j in 0..map_h {
                        for i in 0..map_w {
                            if i == x && j == y {
                                continue;
                            }

                            let o = fov.get_at(i, j);
                            if o < 1.0 {
                                fn fade(x1: u32, y1: u32, x2: u32, y2: u32, _r: u32) -> f32 {
                                    let (x1, y1) = (x1 as i32, y1 as i32);
                                    let (x2, y2) = (x2 as i32, y2 as i32);
                                    let dt = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as f32;
                                    1.0 / (1.0 + dt.sqrt())
                                }

                                let coeff = fade(x, y, i, j, radius);
                                let lum = strength * (1.0 - o) * coeff;
                                illum(&mut tiles, &n_view, i, j, lum);
                            }
                        }
                    }
                }
            }
        }

        RenderedWorldView { normalized_view: n_view, tiles: tiles }
    }
}
