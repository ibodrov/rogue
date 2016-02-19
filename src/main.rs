extern crate sfml;
extern crate time;

mod ui;
mod map;

use ui::UIEvent;

struct Game {
    ui: Box<ui::UI>,
}

impl Game {
    fn new() -> Game {
        Game {
            ui: Box::new(ui::sfml_ui::SFMLUI::new()),
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

                    UIEvent::KeyPressed { code: ui::UIKey::Down, .. } => {
                        println!("down!");
                    },

                    UIEvent::KeyPressed { code: ui::UIKey::Up, .. } => {
                        println!("up!");
                    },

                    UIEvent::KeyPressed { code: ui::UIKey::Right, .. } => {
                        println!("right!");
                    },

                    UIEvent::KeyPressed { code: ui::UIKey::Left, .. } => {
                        println!("left!");
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
