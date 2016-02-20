extern crate sfml;

use std::cmp;

use sfml::graphics::*;
use sfml::window::{ContextSettings, VideoMode, window_style, Key};
use sfml::window::event::Event;
use sfml::system::Vector2f;

use ui::{UI, UIEvent, UIKey};
use map;

pub struct SFMLUI {
    window: RenderWindow,
    map: map::Map,
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
            map: m,
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
        w.draw(&self.map);
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

impl Drawable for map::Map {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
        let va = {
            let m = self;
            let (view_w, view_h) = {
                let Vector2f { x, y } = target.get_view().get_size();
                (x as u32, y as u32)
            };

            let mut va = VertexArray::new().unwrap();
            va.set_primitive_type(PrimitiveType::sfQuads);

            let tile_w = 16;
            let tile_h = 16;

            let (map_w, map_h) = m.size();
            let view_map_w = cmp::min(map_w, view_w / tile_w);
            let view_map_h = cmp::min(map_h, view_h / tile_h);

            for x in 0..view_map_w {
                for y in 0..view_map_h {
                    let tile = m.get_at(x, y);
                    let color = match tile {
                        0 => Color::black(),
                        1 => Color::red(),
                        2 => Color::green(),
                        _ => Color::yellow(),
                    };

                    // +--------+
                    // | 1    2 |
                    // |        |
                    // | 4    3 |
                    // +--------+

                    let (x1, y1) = ( x      * tile_w,  y      * tile_h);
                    let (x2, y2) = ((x + 1) * tile_w,  y      * tile_h);
                    let (x3, y3) = ((x + 1) * tile_w, (y + 1) * tile_h);
                    let (x4, y4) = ( x      * tile_w, (y + 1) * tile_h);

                    fn append(va: &mut VertexArray, x: u32, y: u32, c: &Color) {
                        let v = Vertex::new_with_pos_color(&Vector2f::new(x as f32, y as f32), &c);
                        va.append(&v);
                    }

                    append(&mut va, x1, y1, &color);
                    append(&mut va, x2, y2, &color);
                    append(&mut va, x3, y3, &color);
                    append(&mut va, x4, y4, &color);
                }
            }

            va
        };

        target.draw(&va);
    }
}
