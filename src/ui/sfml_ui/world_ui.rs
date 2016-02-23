use std::rc::Rc;
use std::cell::RefCell;
use sfml::graphics;
use sfml::graphics::{Drawable, RenderTarget, RenderStates, VertexArray, PrimitiveType, Color};
use sfml::system::Vector2f;

use world;
use world::tile;
use world::render;
use world::render::Renderable;
use ui::sfml_ui::utils::vector2f_to_pair_i32;

const TILE_W: u32 = 12;
const TILE_H: u32 = 12;

pub struct WorldUI {
    world: Rc<RefCell<world::World>>,
    ui_view: graphics::View,
}

impl WorldUI {
    pub fn new(w: Rc<RefCell<world::World>>) -> Self {
        WorldUI {
            world: w,
            ui_view: graphics::View::new_init(&Vector2f::new(512.0, 384.0),
                                              &Vector2f::new(1024.0, 768.0)).unwrap(),
        }
    }
}

impl Drawable for WorldUI {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
        let world = self.world.borrow();

        let view = {
            let ui_view = &self.ui_view;

            let (ui_view_w, ui_view_h) = vector2f_to_pair_i32(&ui_view.get_size());
            let (ui_view_x, ui_view_y) = {
                let (x, y) = vector2f_to_pair_i32(&ui_view.get_center());
                (x - ui_view_w / 2, y - ui_view_h / 2)
            };

            let x = ui_view_x / TILE_W as i32;
            let y = ui_view_y / TILE_H as i32;
            let w = ui_view_w as u32 / TILE_W + 1;
            let h = ui_view_h as u32 / TILE_H + 1;

            render::View::new(x, y, 0, w, h)
        };


        let render = world.render(&view);

        target.set_view(&self.ui_view);
        target.draw(&render);
    }
}

impl Drawable for render::RenderedWorldView {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
        let va = {
            // dimensions of the view of the world
            let va_size = self.tiles_count() * 4;
            let mut va = VertexArray::new_init(PrimitiveType::sfQuads, va_size).unwrap();

            let mut vertex_n = 0;
            for (x, y, tile) in self.iter() {
                let color = calculate_color(tile);

                // +--------+
                // | 1    2 |
                // |        |
                // | 4    3 |
                // +--------+

                let (x1, y1) = ( x      * TILE_W,  y      * TILE_H);
                let (x2, y2) = ((x + 1) * TILE_W,  y      * TILE_H);
                let (x3, y3) = ((x + 1) * TILE_W, (y + 1) * TILE_H);
                let (x4, y4) = ( x      * TILE_W, (y + 1) * TILE_H);

                fn update(va: &VertexArray, n: u32, x: u32, y: u32, c: &Color) {
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
