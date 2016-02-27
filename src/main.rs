extern crate time;
extern crate rand;

// our crates
extern crate world;

mod ui;

use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

struct Game {
    world: Rc<RefCell<world::World>>,
    ui: ui::SFMLUI,
}

impl Game {
    fn new() -> Game {
        let w = Rc::new(RefCell::new(world::World::new()));

        Game {
            world: w.clone(),
            ui: ui::SFMLUI::new(w.clone()),
        }
    }

    fn do_loop(&mut self) {
        let ui = &mut self.ui;

        let mut cnt = 0;
        let mut t0 = time::precise_time_s();

        while ui.is_open() {
            loop {
                match ui.poll_event() {
                    ui::Event::NoEvent => break,
                    ui::Event::Closed => return,
                    ui::Event::KeyPressed { code, .. } => {
                        fn move_torch(w: &mut world::World, id: world::EntityId, dx: i32, dy: i32) {
                            let (map_w, map_h) = {
                                let (w, h, _) = w.data().map.size();
                                (w as i32, h as i32)
                            };

                            w.update(|cs| {
                                if let Some(pos) = cs.position.get_mut(&id) {
                                    let (x, y) = {
                                        let mut x = (pos.x as i32) + dx;
                                        if x < 0 {
                                            x = 0;
                                        }

                                        if x >= map_w {
                                            x = map_w - 1;
                                        }

                                        let mut y = (pos.y as i32) + dy;
                                        if y < 0 {
                                            y = 0;
                                        }

                                        if y >= map_h {
                                            y = map_h - 1;
                                        }

                                        (x as u32, y as u32)
                                    };

                                    pos.x = x;
                                    pos.y = y;
                                }
                            });
                        }

                        match code {
                            ui::Key::Space => {
                                let mut w = self.world.borrow_mut();
                                w.data_mut().map.randomize(1, 0);
                            },

                            ui::Key::Down => {
                                let mut w = self.world.borrow_mut();
                                move_torch(&mut w, world::EntityId(0), 0, 1);
                            },

                            ui::Key::Up => {
                                let mut w = self.world.borrow_mut();
                                move_torch(&mut w, world::EntityId(0), 0, -1);
                            },

                            ui::Key::Left => {
                                let mut w = self.world.borrow_mut();
                                move_torch(&mut w, world::EntityId(0), -1, 0);
                            },

                            ui::Key::Right => {
                                let mut w = self.world.borrow_mut();
                                move_torch(&mut w, world::EntityId(0), 1, 0);
                            },

                            ui::Key::W => {
                                let v = &mut ui.world_ui.ui_view;
                                v.move2f(0.0, -5.0);
                            },

                            ui::Key::S => {
                                let v = &mut ui.world_ui.ui_view;
                                v.move2f(0.0, 5.0);
                            },

                            ui::Key::A => {
                                let v = &mut ui.world_ui.ui_view;
                                v.move2f(-5.0, 0.0);
                            },

                            ui::Key::D => {
                                let v = &mut ui.world_ui.ui_view;
                                v.move2f(5.0, 0.0);
                            },

                            ui::Key::Equal => {
                                let mut w = self.world.borrow_mut();
                                let mut rng = rand::thread_rng();
                                let (map_w, map_h, _) = w.data().map.size();
                                world::add_torch(&mut w, rng.gen_range(0, map_w), rng.gen_range(0, map_h), 10);
                            },

                            ui::Key::Dash => {
                                let mut w = self.world.borrow_mut();
                                w.delete_entity(0);
                            },

                            _ => (),
                        }
                    },
                    _ => continue,
                }
            }

            self.world.borrow_mut().tick();
            ui.display();

            cnt += 1;
            let t1 = time::precise_time_s();
            let dt = t1 - t0;
            if dt >= 1.0 {
                println!("FPS: {}", cnt);
                cnt = 0;
                t0 = t1;
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.do_loop();
}
