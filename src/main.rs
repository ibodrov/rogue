extern crate sfml;

mod ui;
mod map;


struct Game {
    ui: Box<ui::SFMLUI>,
    map: map::Map,
}

impl Game {
    fn new() -> Game {
        Game {
            ui: Box::new(ui::SFMLUI::new()),
            map: map::Map::new(128, 128),
        }
    }

    fn do_loop(&mut self) {
        let ui = &mut self.ui;

        while ui.is_open() {
            loop {
                use ui::Event;

                let e = ui.poll_event();
                match e {
                    Event::NoEvent => break,

                    Event::Closed => return,

                    Event::KeyPressed { code: ui::Key::Down, .. } => {
                        println!("down!");
                    },

                    Event::KeyPressed { code: ui::Key::Up, .. } => {
                        println!("up!");
                    },

                    Event::KeyPressed { code: ui::Key::Right, .. } => {
                        println!("right!");
                    },

                    Event::KeyPressed { code: ui::Key::Left, .. } => {
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
