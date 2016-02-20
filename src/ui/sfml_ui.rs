extern crate sfml;

use sfml::graphics::*;
use sfml::window::{ContextSettings, VideoMode, window_style, Key};
use sfml::window::event::Event;
use sfml::system::Vector2f;

use ui::{UI, UIEvent, UIKey};
use map;

struct MapView {
    map: map::Map,
    view: View,
}

impl MapView {
    fn new(map: map::Map, view_size: (u32, u32)) -> MapView {
        let (view_w, view_h) = {
            let (w, h) = view_size;
            (w as f32, h as f32)
        };

        let v = View::new_init(&Vector2f::new(view_w / 2.0, view_h / 2.0),
                               &Vector2f::new(view_w, view_h)).unwrap();

        MapView {
            map: map,
            view: v,
        }
    }
}

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
                fn view_move(mv: &mut MapView, x: f32, y: f32) {
                    let v = &mut mv.view;
                    v.move2f(x, y);
                }

                match code {
                    Key::Down => view_move(&mut self.map_view, 0.0, 5.0),
                    Key::Up => view_move(&mut self.map_view, 0.0, -5.0),
                    Key::Left => view_move(&mut self.map_view, -5.0, 0.0),
                    Key::Right => view_move(&mut self.map_view, 5.0, 0.0),
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

impl Drawable for MapView {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
        target.set_view(&self.view);
        target.draw(&self.map);
    }
}

impl Drawable for map::Map {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
        let va = {
            let m = self;

            let tile_w = 16;
            let tile_h = 16;

            let (view_w, view_h) = {
                let Vector2f { x, y } = target.get_view().get_size();
                (x as i32, y as i32)
            };
            let (view_x, view_y) = {
                let Vector2f { x, y } = target.get_view().get_center();
                (x as i32 - view_w / 2, y as i32 - view_h / 2)
            };

            let view_start_i = view_x / tile_w;
            let view_start_j = view_y / tile_h;

            let (map_w, map_h) = {
                let (w, h) = m.size();
                (w as i32, h as i32)
            };

            let view_end_i = ((view_x + view_w) / tile_w) + 1;
            let view_end_j = ((view_y + view_h) / tile_h) + 1;

            let bounds_check = |i, j| {
                i >= 0 && i < map_w && j >= 0 && j < map_h
            };

            // estimate the size of the vertex array
            let va_cnt = {
                let mut cnt = 0;

                for i in view_start_i..view_end_i {
                    for j in view_start_j..view_end_j {
                        if bounds_check(i, j) {
                            cnt += 1;
                        }
                    }
                }

                cnt * 4
            };
            let mut va = VertexArray::new_init(PrimitiveType::sfQuads, va_cnt).unwrap();

            let mut vertex_n = 0;
            for i in view_start_i..view_end_i {
                for j in view_start_j..view_end_j {
                    if !bounds_check(i, j) {
                        continue;
                    }

                    let tile = m.get_at(i as u32, j as u32);
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

                    let x = i as i32;
                    let y = j as i32;

                    let (x1, y1) = ( x      * tile_w,  y      * tile_h);
                    let (x2, y2) = ((x + 1) * tile_w,  y      * tile_h);
                    let (x3, y3) = ((x + 1) * tile_w, (y + 1) * tile_h);
                    let (x4, y4) = ( x      * tile_w, (y + 1) * tile_h);

                    fn update(va: &VertexArray, n: u32, x: i32, y: i32, c: &Color) {
                        let mut v = va.get_vertex(n);
                        v.0.position.x = x as f32;
                        v.0.position.y = y as f32;
                        v.0.color = c.0;
                    }

                    let n = vertex_n;
                    update(&mut va, n,     x1, y1, &color);
                    update(&mut va, n + 1, x2, y2, &color);
                    update(&mut va, n + 2, x3, y3, &color);
                    update(&mut va, n + 3, x4, y4, &color);
                    vertex_n += 4;
                }
            }

            va
        };

        target.draw(&va);
    }
}
