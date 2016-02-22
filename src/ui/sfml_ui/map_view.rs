extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;
use sfml::graphics::*;
use sfml::system::Vector2f;

use map;
use ui::sfml_ui::utils::vector2f_to_pair;

pub struct MapView {
    map: Rc<RefCell<map::Map>>,
    view: View,
}

impl MapView {
    pub fn new(map: Rc<RefCell<map::Map>>, view_size: (u32, u32)) -> MapView {
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

    pub fn get_view_rect(&self) -> IntRect {
        let v = &self.view;

        let (view_w, view_h) = vector2f_to_pair(&v.get_size());
        let (view_x, view_y) = {
            let (x, y) = vector2f_to_pair(&v.get_center());
            (x - view_w / 2, y - view_h / 2)
        };

        IntRect {
            left: view_x,
            top: view_y,
            width: view_w,
            height: view_h,
        }
    }

    pub fn move_view(&mut self, x: f32, y: f32) {
        self.view.move2f(x, y);
    }
}

impl Drawable for MapView {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
        let va = {
            let tile_w = 16;
            let tile_h = 16;

            let view_rect = self.get_view_rect();

            let view_start_i = view_rect.left / tile_w;
            let view_start_j = view_rect.top / tile_h;

            let m = self.map.borrow();
            let fov = m.fov_at(10, 10, 5);

            let (map_w, map_h) = {
                let (w, h) = m.size();
                (w as i32, h as i32)
            };

            let view_end_i = ((view_rect.left + view_rect.width) / tile_w) + 1;
            let view_end_j = ((view_rect.top + view_rect.height) / tile_h) + 1;

            fn bounds_check(i: i32, j: i32, map_w: i32, map_h: i32) -> bool {
                i >= 0 && i < map_w && j >= 0 && j < map_h
            };

            // estimate the size of the vertex array
            let va_cnt = {
                let mut cnt = 0;

                for i in view_start_i..view_end_i {
                    for j in view_start_j..view_end_j {
                        if bounds_check(i, j, map_w, map_h) {
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
                    if !bounds_check(i, j, map_w, map_h) {
                        continue;
                    }

                    let tile = m.get_at(i as u32, j as u32);
                    let opacity = fov[(i + j * map_w) as usize];
                    let mut color = match tile {
                        0 => Color::black(),
                        1 => Color::red(),
                        2 => Color::green(),
                        _ => Color::yellow(),
                    };

                    let light = (255.0 * (1.0 - opacity)) as u8;
                    color = Color::add(color, Color::new_rgb(light, light, light));

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

        target.set_view(&self.view);
        target.draw(&va);
    }
}
