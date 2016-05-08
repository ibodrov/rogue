use tile_map;
use cfg;
use world;

fn to_vec4(v: [u8; 4]) -> [f32; 4] {
    [v[0] as f32 / 255.0, v[1] as f32 / 255.0, v[2] as f32 / 255.0, v[3] as f32 / 255.0]
}

pub fn update<'a, F>(tile_map: &'a mut tile_map::TileMap,
                 rendered_view: &world::render::RenderedView,
                 converter: F)
    where F: Fn(&world::tile::Tile) -> &'a cfg::MapEntityCfg {

    for (x, y, _, t) in rendered_view.iter() {
        let cfg = converter(t);

        tile_map.set_tile(x, y, tile_map::Tile {
            n: cfg.tile(),
            fg_color: to_vec4(cfg.fg_color()),
            .. Default::default()
        });
    }
}
