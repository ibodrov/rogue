extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;
use self::sfml::graphics;
use self::sfml::graphics::{Drawable, RenderTarget, RenderStates, VertexArray, PrimitiveType, Color};
use self::sfml::system::Vector2f;

use world;
use world::tile;
use world::render;
use world::render::Renderable;
use ui::utils::vector2f_to_pair_i32;

const TILE_W: u32 = 12;
const TILE_H: u32 = 12;

pub struct WorldUI {
    world: Rc<RefCell<world::World>>,
    pub ui_view: graphics::View,
}

pub struct RenderWrapper {
    render: world::render::RenderedView,
    view_delta: (i32, i32),
}

impl WorldUI {
    pub fn new(w: Rc<RefCell<world::World>>) -> Self {
        let screen_w = 1024;
        let screen_h = 786;

        WorldUI {
            world: w,
            ui_view: graphics::View::new_init(&Vector2f::new((screen_w / 2) as f32, (screen_h / 2) as f32),
                                              &Vector2f::new(screen_w as f32, screen_h as f32)).unwrap(),
        }
    }
}

impl Drawable for WorldUI {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
        let world = self.world.borrow();
        let (tile_w, tile_h) = (TILE_W as i32, TILE_H as i32);

        let ui_view = &self.ui_view;
        let (ui_view_w, ui_view_h) = vector2f_to_pair_i32(&ui_view.get_size());

        let (ui_view_x, ui_view_y) = {
            let (x, y) = vector2f_to_pair_i32(&ui_view.get_center());
            (x - ui_view_w / 2, y - ui_view_h / 2)
        };

        let view = {
            let x = ui_view_x / tile_w;
            let y = ui_view_y / tile_h;
            let z = 0;

            let sx = (ui_view_w / tile_w) as u32 + 1;
            let sy = (ui_view_h / tile_h) as u32 + 1;
            let sz = 1;

            render::View::new((x, y, z), (sx, sy, sz))
        };

        let render = world.render(&view);
        if render.tiles_count() == 0 {
            return;
        }

        let wrapper = RenderWrapper {
            render: render,
            view_delta: (ui_view_x.abs(), ui_view_y.abs()),
        };

        target.set_view(&self.ui_view);
        target.draw(&wrapper);
    }
}

impl Drawable for RenderWrapper {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
        let va = {
            // smooth scrolling support
            let (view_dx, view_dy) = self.view_delta;
            let (tile_w, tile_h) = (TILE_W as i32, TILE_H as i32);

            // "vertex array size = "tiles count" * "vertexes per quad"
            let va_size = self.render.tiles_count() * 4;
            let mut va = VertexArray::new_init(PrimitiveType::sfQuads, va_size).unwrap();

            let mut vertex_n = 0;
            for (x, y, _, tile) in self.render.iter() {
                let color = calculate_color(tile);

                // +--------+
                // | 1    2 |
                // |        |
                // | 4    3 |
                // +--------+

                let base_x = x as i32 * tile_w + view_dx;
                let base_y = y as i32 * tile_h + view_dy;

                let (x1, y1) = (base_x,          base_y);
                let (x2, y2) = (base_x + tile_w, base_y);
                let (x3, y3) = (base_x + tile_w, base_y + tile_h);
                let (x4, y4) = (base_x,          base_y + tile_h);

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

            va
        };

        target.draw(&va);
    }
}

fn calculate_color(tile: &tile::Tile) -> Color {
    let mut color = match tile.ground {
        0 => Color::black(),
        1 => Color::red(),
        2 => Color::green(),
        _ => Color::yellow(),
    };

    if let Some(ref effects) = tile.effects {
        for e in effects {
            match e {
                &tile::Effect::Lit(lum) => {
                    let c = (255.0 * lum) as u8;
                    color = Color::add(color, Color::new_rgb(c, c, c));
                }
            }
        }
    }

    color
}
