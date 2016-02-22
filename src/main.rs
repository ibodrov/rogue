extern crate sfml;
extern crate time;
extern crate rand;

mod ui;
mod map;
mod circle_iter;
mod fov;

use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;
use ui::{UIEvent, UIKey};

struct World {
    map: Rc<RefCell<map::Map>>,
}

struct Game {
    world: World,
    ui: Box<ui::UI>,
}

impl Game {
    fn new() -> Game {
        let map = Rc::new(RefCell::new(map::Map::new(128, 128)));

        Game {
            world: World { map: map.clone() },
            ui: Box::new(ui::sfml_ui::SFMLUI::new(map.clone())),
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
                    UIEvent::KeyPressed { code: UIKey::Space } => {
                        let mut rng = rand::thread_rng();
                        let mut m = self.world.map.borrow_mut();
                        let (w, h) = m.size();
                        let x = rng.gen_range(0, w);
                        let y = rng.gen_range(0, h);
                        m.set_at(x, y, 1);
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
