use std::vec::Vec;
use std::cmp;

use world;
use world::tile;

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
struct NormalizedView {
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

        RenderedWorldView { normalized_view: n_view, tiles: tiles }
    }
}
