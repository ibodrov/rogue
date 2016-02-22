extern crate sfml;

mod utils;
mod world_view;

use std::rc::Rc;
use std::cell::RefCell;
use sfml::graphics::*;
use sfml::window::{ContextSettings, VideoMode, window_style, Key};
use sfml::window::event::Event;

use ecs;
use ui::{UI, UIEvent, UIKey};
use ui::sfml_ui::world_view::WorldView;

pub struct SFMLUI {
    window: RenderWindow,
    world_view: WorldView,
}

impl SFMLUI {
    pub fn new(world: Rc<RefCell<ecs::World>>) -> Self {
        let s = ContextSettings::default();
        let w = RenderWindow::new(VideoMode::new_init(1024, 768, 32),
                                  "rogue",
                                  window_style::TITLEBAR | window_style::CLOSE,
                                  &s).unwrap();

        // w.set_vertical_sync_enabled(true);

        SFMLUI {
            window: w,
            world_view: WorldView::new(world, (1024, 768)),
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
        w.draw(&self.world_view);
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
            Key::Space => UIKey::Space,
            _ => UIKey::Unknown,
        }
    }
}
