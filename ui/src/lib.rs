#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate time;
extern crate tex_atlas;
extern crate rand;
#[macro_use]
extern crate log;
extern crate world;

mod tile_map;
mod cfg;
mod world_view;

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

fn randomize_map(map: &mut world::map::Map) {
    let nothing = 0;
    let wall = 1;

    let (mx, my, mz) = map.size();
    let mut rng = rand::thread_rng();

    map.fill(nothing);

    for z in 0..mz {
        // top and bottom wall
        for x in 0..mx {
            map[(x, 0, z)] = wall;
            map[(x, my - 1, z)] = wall;
        }

        // left and right wall
        for y in 0..my {
            map[(0, y, z)] = wall;
            map[(mx - 1, y, z)] = wall;
        }

        // random boxes
        let cnt = 100;
        for _ in 0..cnt {
            let x = rng.gen_range(1, mx - 1);
            let y = rng.gen_range(1, my - 1);
            let w = rng.gen_range(2, 5);
            let h = rng.gen_range(2, 5);

            for j in y..y+h {
                for i in x..x+w {
                    if i >= mx || j >= my {
                        continue;
                    }

                    map[(i, j, z)] = wall;
                }
            }
        }
    }
}

pub fn start() {
    use glium::{DisplayBuild, Surface};
    use glium::glutin::{Event, VirtualKeyCode, ElementState};

    let mut world = world::World::default();
    randomize_map(world.map_mut());

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build_glium()
        .unwrap();

    let cfg = cfg::load(&display, "assets/ui.toml");
    let visible_tile_size = cfg.map_cfg().visible_tile_size();
    let tex_atlas = cfg.map_cfg().atlas();

    let dwarf_cfg = cfg.map_cfg().entities().get("dwarf").unwrap();
    let grass_cfg = cfg.map_cfg().entities().get("grass").unwrap();
    let wall_cfg = cfg.map_cfg().entities().get("wall").unwrap();

    let map_size = (SCREEN_WIDTH / visible_tile_size.0, SCREEN_HEIGHT / visible_tile_size.1);
    let mut viewport = Viewport { position: (0, 0), size: (SCREEN_WIDTH, SCREEN_HEIGHT) };
    let mut tile_map = tile_map::TileMap::new(&display, map_size, visible_tile_size, &tex_atlas);

    let mut t0 = time::precise_time_s();
    let mut frames = 0;

    loop {
        {
            for ev in display.poll_events() {
                match ev {
                    Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return,
                    Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                        match code {
                            VirtualKeyCode::W => {
                                world.send_player_command(world::PlayerCommand::MoveUp);
                            },
                            VirtualKeyCode::S => {
                                world.send_player_command(world::PlayerCommand::MoveDown);
                            },
                            VirtualKeyCode::A => {
                                world.send_player_command(world::PlayerCommand::MoveLeft);
                            },
                            VirtualKeyCode::D => {
                                world.send_player_command(world::PlayerCommand::MoveRight);
                            },
                            _ => (),
                        }
                    },
                    Event::Resized(w, h) => {
                        let map_size = (w / visible_tile_size.0, h / visible_tile_size.1);
                        viewport = Viewport { position: (0, 0), size: (w, h) };
                        tile_map = tile_map::TileMap::new(&display, map_size, visible_tile_size, &tex_atlas);
                    },
                    _ => (),
                }
            }
        }

        {
            let view = world::render::View {
                size: (map_size.0, map_size.1, 1),
                .. Default::default()
            };
            let rendered_view = world::render::render(&mut world, &view);
            let converter = move |t: &world::tile::Tile| {
                if let Some(ref fx) = t.effects {
                    for e in fx {
                        if let world::tile::Effect::Marked(_) = *e {
                            return dwarf_cfg;
                        }
                    }
                }

                match t.ground {
                    0 => grass_cfg,
                    1 => wall_cfg,
                    _ => panic!("Unknown cell type: {:?}", t),
                }
            };
            world_view::update(&mut tile_map, &rendered_view, converter);
        }

        world.tick();

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
