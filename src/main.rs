extern crate sfml;
extern crate time;

mod ui;
mod circle_iter;
mod fov;
mod map;
mod ecs;

use std::rc::Rc;
use std::cell::RefCell;
use ui::{UIEvent, UIKey};

struct Game {
    world: Rc<RefCell<ecs::World>>,
    ui: Box<ui::UI>,
}

impl Game {
    fn new() -> Game {
        let w = Rc::new(RefCell::new(ecs::World::new()));

        Game {
            world: w.clone(),
            ui: Box::new(ui::sfml_ui::SFMLUI::new(w.clone())),
        }
    }

    fn do_loop(&mut self) {
        let ui = &mut self.ui;

        let mut cnt = 0;
        let mut t0 = time::precise_time_s();

        while ui.is_open() {
            while let Some(e) = ui.poll_event() {
                match e {
                    UIEvent::Closed => return,
                    UIEvent::KeyPressed { code } => {
                        fn move_torch(w: &mut ecs::World, id: ecs::EntityId, dx: i32, dy: i32) {
                            let (map_w, map_h) = {
                                let (w, h) = w.map().size();
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
                            UIKey::Space => {
                                let mut w = self.world.borrow_mut();
                                w.map_mut().randomize();
                            },

                            UIKey::Down => {
                                let mut w = self.world.borrow_mut();
                                move_torch(&mut w, ecs::EntityId(0), 0, 1);
                            },

                            UIKey::Up => {
                                let mut w = self.world.borrow_mut();
                                move_torch(&mut w, ecs::EntityId(0), 0, -1);
                            },

                            UIKey::Left => {
                                let mut w = self.world.borrow_mut();
                                move_torch(&mut w, ecs::EntityId(0), -1, 0);
                            },

                            UIKey::Right => {
                                let mut w = self.world.borrow_mut();
                                move_torch(&mut w, ecs::EntityId(0), 1, 0);
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
