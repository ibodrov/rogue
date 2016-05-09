use std::sync::{Arc, Mutex};
use specs;
use tile;
use ::WorldContext;

/// Defines the size of the rendered part of the world.
pub struct View {
    pub position: (u32, u32, u32),
    pub size: (u32, u32, u32),
}

impl Default for View {
    fn default() -> Self {
        View {
            position: (0, 0, 0),
            size: (10, 10, 1),
        }
    }
}

/// A rendered portion of the world.
pub struct RenderedView {
    size: (u32, u32, u32),
    tiles: Vec<tile::Tile>,
}

impl RenderedView {
    pub fn iter(&self) -> tile::TilesIter {
        tile::TilesIter::new(self.size, self.tiles.iter())
    }

    pub fn size(&self) -> (u32, u32, u32) {
        self.size
    }
}

pub type ViewHolder = Arc<Mutex<View>>;
pub type RenderedViewHolder = Arc<Mutex<Option<RenderedView>>>;

pub struct RenderingSystem {
    view: ViewHolder,
    render: RenderedViewHolder,
}

impl RenderingSystem {
    pub fn new(render: RenderedViewHolder, view: ViewHolder) -> Self {
        RenderingSystem {
            view: view,
            render: render,
        }
    }

    pub fn last_rendered_view(&self) -> &RenderedViewHolder {
        &self.render
    }
}

impl specs::System<WorldContext> for RenderingSystem {
    fn run(&mut self, arg: specs::RunArg, ctx: WorldContext) {
        use std::cmp::min;
        use components::{Position, Visible};
        use tile::Effect;
        use specs::Join;

        let view = &self.view.lock().unwrap();

        let (mut tiles, position, size) = {
            let map = &ctx.map.lock().unwrap();
            let (map_size_x, map_size_y, map_size_z) = map.size();

            let (start_x, start_y, start_z) = view.position;
            let (view_size_x, view_size_y, view_size_z) = view.size;
            let (end_x, end_y, end_z) = (
                min(start_x + view_size_x, map_size_x),
                min(start_y + view_size_y, map_size_y),
                min(start_z + view_size_z, map_size_z),
            );

            let mut tiles = Vec::new();

            for z in start_z..end_z {
                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let m = map[(x, y, z)];
                        tiles.push(tile::Tile::new(m));
                    }
                }
            }

            (tiles, view.position, (end_x - start_x, end_y - start_y, end_z - start_z))
        };

        let (pos_es, vis_es) = arg.fetch(|w| {
            (w.read::<Position>(), w.read::<Visible>())
        });

        for (pos, vis) in (&pos_es, &vis_es).iter() {
            let &Position { x, y, z } = pos;

            if x < position.0 || x >= position.0 + size.0 {
                continue;
            }
            if y < position.1 || y >= position.1 + size.1 {
                continue;
            }
            if z < position.2 || z >= position.2 + size.2 {
                continue;
            }

            let idx = (x + y * size.0 + z * size.0 * size.1) as usize;
            let t = &mut tiles[idx];
            t.add_effect(Effect::Marked(vis.mark));
        }

        let render = RenderedView {
            size: size,
            tiles: tiles,
        };

        *self.render.lock().unwrap() = Some(render);
    }
}
