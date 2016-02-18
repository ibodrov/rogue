extern crate sfml;

pub use sfml::graphics::Events;
pub use sfml::window::event::Event;
pub use sfml::window::Key;

use sfml::graphics::{RenderWindow, RenderTarget, Color};
use sfml::window::{ContextSettings, VideoMode, window_style};

pub struct SFMLUI {
    window: RenderWindow,
}

impl SFMLUI {
    pub fn new() -> Self {
        let s = ContextSettings::default();
        let w = RenderWindow::new(VideoMode::new_init(1024, 768, 32),
                                  "rogue",
                                  window_style::TITLEBAR | window_style::CLOSE,
                                  &s).unwrap();

        SFMLUI {
            window: w,
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn events(&mut self) -> Events {
        self.window.events()
    }

    pub fn poll_event(&mut self) -> Event {
        self.window.poll_event()
    }

    pub fn display(&mut self) {
        let w = &mut self.window;
        w.clear(&Color::black());
        w.display();
    }
}
