use std::vec::Vec;
use std::cmp;

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
        NormalizedView::new(x, y, self.level, self.width, self.height)
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
    size: (u32, u32),
    tiles: Vec<tile::Tile>,
}

impl RenderedWorldView {
    pub fn tiles_count(&self) -> u32 {
        self.tiles.len() as u32
    }

    pub fn iter(&self) -> tile::TilesIter {
        tile::TilesIter::new(self.size.0, self.tiles.iter())
    }

    pub fn get_abs_mut<'a>(tiles: &'a mut Vec<tile::Tile>, size: (u32, u32), v: &NormalizedView, x: u32, y: u32, level: u32) -> Option<&'a mut tile::Tile> {
        let (w, h) = size;
        if x < v.x || x >= v.x + w || y < v.y || y >= v.y + h {
            return None;
        }

        let (nx, ny) = (x - v.x, y - v.y);
        let n = (nx + ny * w + level * w * h) as usize;

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

        let map = &self.data().map;
        let (map_w, map_h, _) = map.size();

        // determine the range of tiles
        let map_start_i = cmp::max(view.x, 0) as u32;
        let map_end_i = cmp::min(map_start_i + view_w, map_w);
        let map_start_j = cmp::max(view.y, 0) as u32;
        let map_end_j = cmp::min(map_start_j + view_h, map_h);

        // actual dimensions of the tiles array
        let actual_size = (map_end_i - map_start_i, map_end_j - map_start_j);

        // convert map's data into tiles
        for j in map_start_j..map_end_j {
            for i in map_start_i..map_end_i {
                let g = map.get_at(i, j, view_level);
                let t = tile::Tile::new(*g);
                tiles.push(t);
            }
        }

        debug_assert!(actual_size.0 * actual_size.1 == tiles.len() as u32,
                      "Invalid actual size of the tiles vector. Vector size: {}, dimensions: {:?}", tiles.len(), actual_size);

        let is_visible = |x: u32, y: u32, r: u32| {
            let (x, y, r) = (x as i32, y as i32, r as i32);
            let (m_sx, m_sy) = ((map_start_i as i32), (map_start_j as i32));
            let (m_ex, m_ey) = ((map_end_i as i32), (map_end_j as i32));
            (x + r >= m_sx && x - r < m_ex) && (y + r >= m_sy && y - r < m_ey)
        };

        // render entities
        let cs = &self.data().components;
        for e in &self.data().entities {
            if let Some(ref g) = cs.glow.get(e) {
                if let Some(&components::Position { x, y }) = cs.position.get(e) {
                    // we got a torch
                    if !is_visible(x, y, g.radius) {
                        continue;
                    }

                    let illum = |ts: &mut Vec<tile::Tile>, v: &NormalizedView, x: u32, y: u32, lum: f32| {
                        // TODO level?
                        if let Some(t) = RenderedWorldView::get_abs_mut(ts, actual_size, v, x, y, 0) {
                            t.add_effect(tile::Effect::Lit(lum));
                        } 
                    };

                    let (lm_w, lm_h) = g.light_map_size;

                    for j in 0..lm_h {
                        for i in 0..lm_w {
                            let o = g.get_at(i, j);
                            if o >= 1.0 {
                                continue;
                            }

                            let mx = x + i - lm_w / 2;
                            let my = y + j - lm_h / 2;

                            fn fade(x1: u32, y1: u32, x2: u32, y2: u32, _r: u32) -> f32 {
                                let (x1, y1) = (x1 as i32, y1 as i32);
                                let (x2, y2) = (x2 as i32, y2 as i32);
                                let dt = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as f32;
                                1.0 / (1.0 + dt.sqrt())
                            }

                            let coeff = fade(x, y, mx, my, g.radius);
                            let lum = 1.0 * (1.0 - o) * coeff;
                            illum(&mut tiles, &n_view, mx, my, lum);
                        }
                    }
                }
            }
        }

        RenderedWorldView { size: actual_size, tiles: tiles }
    }
}
