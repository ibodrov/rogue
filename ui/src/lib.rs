#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate time;
extern crate world;
extern crate tex_atlas;
extern crate rand;

mod tile_map;

use tile_map::Renderable;
use rand::Rng;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

pub fn start() {
    use glium::{DisplayBuild, Surface};
    use glium::glutin::{Event, VirtualKeyCode, ElementState};

    //let mut world = world::World::new();

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build_glium()
        .unwrap();

    //let mut world_view = world_view::WorldUI::new(&display);
    let tex_atlas = tex_atlas::load(&display, std::path::Path::new("assets/atlas.toml")).unwrap();

    let tile_size = (8, 10);
    let map_size = (SCREEN_WIDTH / tile_size.0, SCREEN_HEIGHT / tile_size.1);
    let mut tile_map = tile_map::TileMap::new(&display, tile_size, map_size, (SCREEN_WIDTH, SCREEN_HEIGHT), tex_atlas);

    let mut t0 = time::precise_time_s();
    let mut frames = 0;

    loop {
        {
            //use world::systems::{KeyboardCommand};

            for ev in display.poll_events() {
                match ev {
                    Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return,
                    Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                        match code {
                            VirtualKeyCode::Space => {
                                let (mw, mh) = tile_map.map_size();
                                let mut rng = rand::thread_rng();
                                for _ in 0..10 {
                                    let x = rng.gen_range(0, mw);
                                    let y = rng.gen_range(0, mh);
                                    let t = rng.gen_range(0, 16*16);
                                    tile_map.set_tile(x, y, tile_map::Tile(t));
                                }
                            },
                            _ => (),
                        }

                        /*
                        let view = &mut world_view.view;

                        let scroll_speed = 10;
                        match code {
                            VirtualKeyCode::Up => view.y -= scroll_speed,
                            VirtualKeyCode::Down => view.y += scroll_speed,
                            VirtualKeyCode::Left => view.x -= scroll_speed,
                            VirtualKeyCode::Right => view.x += scroll_speed,
                            VirtualKeyCode::Comma => {
                                view.z -= 1;
                                if view.z < 0 {
                                    view.z = 0;
                                }
                            },
                            VirtualKeyCode::Period => {
                                view.z += 1;
                                if view.z >= 3 {
                                    view.z = 2;
                                }
                            },

                            VirtualKeyCode::W => {
                                let control = &mut world.systems.control;
                                control.add(KeyboardCommand::UP);
                            },
                            VirtualKeyCode::S => {
                                let control = &mut world.systems.control;
                                control.add(KeyboardCommand::DOWN);
                            },
                            VirtualKeyCode::A => {
                                let control = &mut world.systems.control;
                                control.add(KeyboardCommand::LEFT);
                            },
                            VirtualKeyCode::D => {
                                let control = &mut world.systems.control;
                                control.add(KeyboardCommand::RIGHT);
                            },

                            VirtualKeyCode::Space => world.data_mut().map.randomize(1, 0),
                            _ => (),
                        }
                        */
                    },
                    _ => (),
                }
            }
        }

        //world.tick();

        let mut target = display.draw();
        //world_view.render(&display, &mut target, (SCREEN_WIDTH, SCREEN_HEIGHT), &world);
        tile_map.render(&display, &mut target);
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
