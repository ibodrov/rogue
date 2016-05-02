#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate time;
extern crate tex_atlas;
extern crate rand;

mod tile_map;
mod cfg;

use rand::Rng;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 760;

struct Viewport {
    position: (u32, u32),
    size: (u32, u32),
}

trait Renderable {
    fn render<F, S>(&self, display: &F, target: &mut S, viewport: &Viewport)
        where F: glium::backend::Facade, S: glium::Surface;
}

fn to_vec4(v: [u8; 4]) -> [f32; 4] {
    [v[0] as f32 / 255.0, v[1] as f32 / 255.0, v[2] as f32 / 255.0, v[3] as f32 / 255.0]
}

pub fn start() {
    use glium::{DisplayBuild, Surface};
    use glium::glutin::{Event, VirtualKeyCode, ElementState};

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build_glium()
        .unwrap();

    let cfg = cfg::load(&display, "assets/ui.toml");
    let tex_atlas = cfg.map_cfg().atlas();

    let dwarf_cfg = cfg.map_cfg().entities().get("dwarf").unwrap();
    let grass_cfg = cfg.map_cfg().entities().get("grass").unwrap();

    //let tex_atlas = tex_atlas::load(&display, std::path::Path::new("assets/atlas.toml")).unwrap();
    //let (tiles_cols, tiles_rows) = tex_atlas.tile_count();

    let tile_size = tex_atlas.tile_size();
    let map_size = (SCREEN_WIDTH / tile_size.0, SCREEN_HEIGHT / tile_size.1);
    //let map_size = (16, 16);
    let mut viewport = Viewport { position: (0, 0), size: (SCREEN_WIDTH, SCREEN_HEIGHT) };
    let mut tile_map = tile_map::TileMap::new(&display, map_size, &tex_atlas);

    let mut t0 = time::precise_time_s();
    let mut frames = 0;

    loop {
        {
            for ev in display.poll_events() {
                match ev {
                    Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return,
                    Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                        match code {
                            VirtualKeyCode::Space => {
                                let (mw, mh) = tile_map.size();
                                let mut rng = rand::thread_rng();
                                for _ in 0..10000 {
                                    let x = rng.gen_range(0, mw);
                                    let y = rng.gen_range(0, mh);
                                    let t = {
                                        if rng.gen_weighted_bool(100) {
                                            dwarf_cfg
                                        } else {
                                            grass_cfg
                                        }
                                    };

                                    tile_map.set_tile(x, y, tile_map::Tile {
                                        n: t.tile(),
                                        fg_color: to_vec4(t.fg_color()),
                                        bg_color: [0.0, 0.0, 0.0],
                                    });
                                }
                            },
                            _ => (),
                        }
                    },
                    Event::Resized(w, h) => {
                        let map_size = (w / tile_size.0, h / tile_size.1);
                        viewport = Viewport { position: (0, 0), size: (w, h) };
                        tile_map = tile_map::TileMap::new(&display, map_size, &tex_atlas);
                    },
                    _ => (),
                }
            }
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        tile_map.render(&display, &mut target, &viewport);
        target.finish().unwrap();

        frames += 1;

        let t1 = time::precise_time_s();
        if t1 - t0 >= 1.0 {
            t0 = t1;
            display.get_window().unwrap().set_title(&format!("~{} FPS", frames));
            frames = 0;
        }
    }
}
