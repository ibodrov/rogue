extern crate sfml;

mod ui;
mod map;

use ui::UIEvent;

struct Game {
    ui: Box<ui::UI>,
    map: map::Map,
}

impl Game {
    fn new() -> Game {
        Game {
            ui: Box::new(ui::sfml_ui::SFMLUI::new()),
            map: map::Map::new(128, 128),
        }
    }

    fn do_loop(&mut self) {
        let ui = &mut self.ui;

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
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.do_loop();
}
