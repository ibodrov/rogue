extern crate sfml;

mod utils;
mod world_ui;

use std::rc::Rc;
use std::cell::RefCell;
use self::sfml::graphics::*;
use self::sfml::window::{ContextSettings, VideoMode, window_style};
pub use self::sfml::window::event::Event;
pub use self::sfml::window::Key;

use world;

pub struct SFMLUI {
    window: RenderWindow,
    world_ui: world_ui::WorldUI,
}

impl SFMLUI {
    pub fn new(world: Rc<RefCell<world::World>>) -> Self {
        let s = ContextSettings::default();
        let w = RenderWindow::new(VideoMode::new_init(1024, 768, 32),
                                  "rogue",
                                  window_style::TITLEBAR | window_style::CLOSE,
                                  &s).unwrap();

        // w.set_vertical_sync_enabled(true);

        SFMLUI {
            window: w,
            world_ui: world_ui::WorldUI::new(world),
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn poll_event(&mut self) -> Event {
        self.window.poll_event()
    }

    pub fn display(&mut self) {
        let w = &mut self.window;
        w.clear(&Color::black());
        w.draw(&self.world_ui);
        w.display();
    }
}