extern crate sfml;

mod map_view;
mod utils;

use sfml::graphics::*;
use sfml::window::{ContextSettings, VideoMode, window_style, Key};
use sfml::window::event::Event;

use map;
use ui::{UI, UIEvent, UIKey};
use ui::sfml_ui::map_view::MapView;

pub struct SFMLUI {
    window: RenderWindow,
    map_view: MapView,
}

impl SFMLUI {
    pub fn new() -> Self {
        let s = ContextSettings::default();
        let w = RenderWindow::new(VideoMode::new_init(1024, 768, 32),
                                  "rogue",
                                  window_style::TITLEBAR | window_style::CLOSE,
                                  &s).unwrap();

        let m = map::Map::new(128, 128);

        SFMLUI {
            window: w,
            map_view: MapView::new(m, (1024, 768)),
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
            Event::KeyPressed { code, .. } => {
                let mv = &mut self.map_view;
                let s = 10.0;
                match code {
                    Key::Down => mv.move_view(0.0, s),
                    Key::Up => mv.move_view(0.0, -s),
                    Key::Left => mv.move_view(-s, 0.0),
                    Key::Right => mv.move_view(s, 0.0),
                    _ => (),
                }

                Some(UIEvent::KeyPressed { code: UIKey::from(code) })
            },
            _ => Some(UIEvent::Unknown),
        }
    }

    fn display(&mut self) {
        let w = &mut self.window;
        w.clear(&Color::black());
        w.draw(&self.map_view);
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
