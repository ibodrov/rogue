extern crate time;

mod ui;
mod circle_iter;
mod fov;
mod world;

use std::rc::Rc;
use std::cell::RefCell;

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
                                let (w, h, _) = w.map().size();
                                (w as i32, h as i32)
                            };

                            w.update(|cs| {
                                if let Some(pos) = cs.positions.get_mut(&id) {
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
                                w.map_mut().randomize(1, 0);
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

                            _ => (),
                        }
                    },
                    _ => continue,
                }
            }

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
