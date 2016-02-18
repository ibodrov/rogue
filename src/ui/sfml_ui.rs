extern crate sfml;

use sfml::graphics::{RenderWindow, RenderTarget, Color};
use sfml::window::{ContextSettings, VideoMode, window_style, Key};
use sfml::window::event::Event;

use ui::{UI, UIEvent, UIKey};

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
}

impl UI for SFMLUI {
    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn poll_event(&mut self) -> Option<UIEvent> {
        match self.window.poll_event() {
            Event::NoEvent => None,
            Event::Closed => Some(UIEvent::Closed),
            Event::KeyPressed { code, .. } => Some(UIEvent::KeyPressed { code: UIKey::from(code) }),
            _ => Some(UIEvent::Unknown),
        }
    }

    fn display(&mut self) {
        let w = &mut self.window;
        w.clear(&Color::black());
        w.display();
    }
}

impl From<Key> for UIKey {
    fn from(k: Key) -> Self {
        match k {
            Key::Up => UIKey::Up,
            Key::Down => UIKey::Down,
            Key::Left => UIKey::Left,
            Key::Right => UIKey::Right,
            _ => UIKey::Unknown,
        }
    }
}
