use std::vec::Vec;
use std::cmp;

use super::World;
use tile;

/// The "window" to the map, described in absolute coordinates.
#[derive(Debug, Clone, Copy)]
pub struct View {
    position: (i32, i32, i32),
    size: (u32, u32, u32),
}

impl View {
    pub fn new(position: (i32, i32, i32), size: (u32, u32, u32)) -> Self {
        View { position: position, size: size }
    }
}

pub struct RenderedView {
    // actual dimensions of the tiles vector (x size, y size, z size)
    size: (u32, u32, u32),

    tiles: Vec<tile::Tile>,
}

impl RenderedView {
    pub fn iter(&self) -> tile::TilesIter {
        tile::TilesIter::new(self.size, self.tiles.iter())
    }

    pub fn tiles_count(&self) -> u32 {
        self.tiles.len() as u32
    }

    pub fn get_mut(tiles: &mut Vec<tile::Tile>, size: (u32, u32, u32), x: u32, y: u32, z: u32) -> &mut tile::Tile {
        let n = (x + y * size.0 + z * size.0 * size.1) as usize;
        debug_assert!(n < tiles.len(),
                      "Invalid tile index. \
                       Tiles count: {}, index: {}, size: {:?}, coordinates: {}, {}, {}",
                      tiles.len(), n, size, x, y, z);

        &mut tiles[n]
    }
}

pub trait Renderable {
    fn render(&self, view: &View) -> RenderedView;
}

impl Renderable for World {
    fn render(&self, view: &View) -> RenderedView {
        let map = &self.data().map;

        let (view_x, view_y, view_z) = view.position;
        let (view_x_size, view_y_size, view_z_size) = view.size;
        let (map_max_x, map_max_y, map_max_z) = map.size();

        fn norm(view_v: i32, view_size: u32, map_size: u32) -> (u32, u32) {
            // TODO better handling of negative `view_v`
            let start = cmp::max(0, view_v);
            let end = cmp::min(map_size as i32, cmp::max(0, view_v + view_size as i32));
            (start as u32, end as u32)
        }

        // normalize dimensions of the visible part of the map
        let (n_start_x, n_end_x) = norm(view_x, view_x_size, map_max_x);
        let (n_start_y, n_end_y) = norm(view_y, view_y_size, map_max_y);
        let (n_start_z, n_end_z) = norm(view_z, view_z_size, map_max_z);

        fn size(s: u32, e: u32) -> u32 {
            if e < s {
                0
            } else {
                e - s
            }
        }

        let n_size_x = size(n_start_x, n_end_x);
        let n_size_y = size(n_start_y, n_end_y);
        let n_size_z = size(n_start_z, n_end_z);

        let max_tiles = (n_size_x * n_size_y * n_size_z) as usize;
        let mut tiles = Vec::with_capacity(max_tiles);

        for z in n_start_z..n_end_z {
            for y in n_start_y..n_end_y {
                for x in n_start_x..n_end_x {
                    let g = map.get_at(x, y, z);
                    let t = tile::Tile::new(*g);
                    tiles.push(t);
                }
            }
        }

        debug_assert!(max_tiles == tiles.len(),
                      "Invalid size of the tiles vector. Expected: {}, actual: {}", max_tiles, tiles.len());

        let is_visible = move |x: u32, y: u32, _z: u32, r: u32| {
            let (x, y, r) = (x as i32, y as i32, r as i32);
            let (m_sx, m_sy) = ((n_start_x as i32), (n_start_y as i32));
            let (m_ex, m_ey) = ((n_end_x as i32), (n_end_y as i32));
            (x + r >= m_sx && x - r < m_ex) && (y + r >= m_sy && y - r < m_ey)
        };

        // render entities
        for e in &self.data().entities {
            render_glow(e, &self.data.components, &is_visible, &mut tiles,
                        (n_start_x, n_start_y, n_start_z),
                        (n_size_x, n_size_y, n_size_z));

        }

        RenderedView { size: (n_size_x, n_size_y, n_size_z), tiles: tiles }
    }
}

type CheckFn = Fn(u32, u32, u32, u32) -> bool;

fn render_glow(e: &super::ecs::EntityId,
               cs: &super::ecs::Data,
               is_visible: &CheckFn,
               tiles: &mut Vec<tile::Tile>,
               norm_view: (u32, u32, u32),
               view_size: (u32, u32, u32)) {

    use components::{Glow, Position};

    if let Some((ref g, &Position { x, y, z })) = cs.join::<Glow, Position>(e) {
        // we got a torch
        if !is_visible(x, y, z, g.radius) {
            return;
        }

        let (n_start_x, n_start_y, _) = norm_view;
        let (n_size_x, n_size_y, n_size_z) = view_size;

        let illum = |ts: &mut Vec<tile::Tile>, x: u32, y: u32, z: u32, lum: f32| {
            // TODO level?
            if !is_visible(x, y, z, 0) {
                return;
            }

            let x = x - n_start_x;
            let y = y - n_start_y;
            let t = RenderedView::get_mut(ts, (n_size_x, n_size_y, n_size_z), x, y, z);
            t.add_effect(tile::Effect::Lit(lum));
        };

        let (lm_w, lm_h) = g.light_map_size;

        for j in 0..lm_h {
            for i in 0..lm_w {
                let o = g.get_at(i, j);
                if o >= 1.0 {
                    continue;
                }

                if x + i < lm_w / 2 || y + j < lm_h / 2 {
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
                illum(tiles, mx, my, z, lum);
            }
        }
    }
}
