use tile_map;
use cfg;
use world;

fn to_vec4(v: [u8; 4]) -> [f32; 4] {
    [v[0] as f32 / 255.0, v[1] as f32 / 255.0, v[2] as f32 / 255.0, v[3] as f32 / 255.0]
}

pub enum TileVariant {
    Entity(cfg::ui::UiMapTileCfg),
    Simple(u8),
}

pub fn update<F>(tile_map: &mut tile_map::TileMap,
                 rendered_view: &world::systems::RenderedView,
                 converter: F)
    where F: Fn(&world::tile::Tile) -> TileVariant {

    for (x, y, _, t) in rendered_view.iter() {
        match converter(t) {
            TileVariant::Simple(t) => {
                tile_map.set_tile(x, y, tile_map::Tile {
                    n: t,
                    fg_color: to_vec4([255, 255, 255, 255]),
                    .. Default::default()
                })
            },
            TileVariant::Entity(cfg) => {
                tile_map.set_tile(x, y, tile_map::Tile {
                    n: cfg.tile_n as u8, // FIXME get rid of casting
                    fg_color: to_vec4(cfg.fg),
                    .. Default::default()
                });
            },
        }
    }
}
