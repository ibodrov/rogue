#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate time;
extern crate world;

use world::render::Renderable;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const TILE_WIDTH: i32 = 8;
const TILE_HEIGHT: i32 = 8;

#[derive(Copy, Clone)]
struct Vertex {
    position: [i32; 2],
}

implement_vertex!(Vertex, position);

#[derive (Copy, Clone)]
struct Instance {
    screen_position: [i32; 2],
    color: [f32; 3],
}

implement_vertex!(Instance, screen_position, color);

struct View {
    x: i32,
    y: i32,
}

const QUAD_INDICES: &'static [u16] = &[0u16, 1, 2, 1, 3, 2];
const QUAD: &'static [Vertex] = &[
    Vertex { position: [0,          TILE_HEIGHT], },
    Vertex { position: [TILE_WIDTH, TILE_HEIGHT], },
    Vertex { position: [0,          0],           },
    Vertex { position: [TILE_WIDTH, 0],           },
];

pub fn start() {
    use glium::{DisplayBuild, Surface};
    use glium::glutin::{Event, VirtualKeyCode, ElementState};

    let mut world = world::World::new();

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build_glium()
        .unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &QUAD).unwrap();

    let index_buffer = glium::IndexBuffer::new(&display,
                                               glium::index::PrimitiveType::TrianglesList,
                                               QUAD_INDICES).unwrap();

    let vertex_shader = r#"
        #version 150

        in ivec2 position;
        in ivec2 screen_position;
        in vec3 color;

        uniform mat4 matrix;

        out vec3 v_Color;

        void main() {
            gl_Position = matrix * vec4(position + screen_position, 0.0, 1.0);
            v_Color = color;
        }
    "#;

    let fragment_shader = r#"
        #version 150

        in vec3 v_Color;
        out vec4 color;

        void main() {
            color = vec4(v_Color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();
    let proj: [[f32; 4]; 4] = cgmath::ortho(0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32, 0.0, -1.0, 1.0).into();

    let mut view = View { x: 0, y: 0 };

    let mut t0 = time::precise_time_s();
    let mut frames = 0;

    loop {
        for ev in display.poll_events() {
            match ev {
                Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                    let scroll_speed = 10;
                    match code {
                        VirtualKeyCode::Up => view.y -= scroll_speed,
                        VirtualKeyCode::Down => view.y += scroll_speed,
                        VirtualKeyCode::Left => view.x -= scroll_speed,
                        VirtualKeyCode::Right => view.x += scroll_speed,
                        VirtualKeyCode::Space => world.data_mut().map.randomize(1, 0),
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        world.tick();

        let uniforms = uniform! {
            matrix: proj,
        };

        let instances = {
            let data = {
                let world_view = {
                    let x = view.x / TILE_WIDTH;
                    let y = view.y / TILE_HEIGHT;
                    let z = 0;

                    let sx = SCREEN_WIDTH / TILE_WIDTH as u32 + 1;
                    let sy = SCREEN_HEIGHT / TILE_HEIGHT as u32 + 1;
                    let sz = 1;

                    world::render::View::new((x, y, z), (sx, sy, sz))
                };

                // smooth scrolling support
                let (view_dx, view_dy) = get_view_delta(&view);

                let render = world.render(&world_view);
                let mut v = Vec::with_capacity(render.tiles_count() as usize);

                for (x, y, _, tile) in render.iter() {
                    let x = x as i32 * TILE_WIDTH + view_dx;
                    let y = y as i32 * TILE_HEIGHT + view_dy;

                    let color = calculate_color(&tile);
                    v.push(Instance { screen_position: [x, y], color: color });
                }

                v
            };

            glium::vertex::VertexBuffer::dynamic(&display, &data).unwrap()
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw((&vertex_buffer, instances.per_instance().unwrap()), &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        frames += 1;

        let t1 = time::precise_time_s();
        if t1 - t0 >= 1.0 {
            t0 = t1;
            display.get_window().unwrap().set_title(&format!("~{} FPS", frames));
            frames = 0;
        }
    }
}

fn get_view_delta(v: &View) -> (i32, i32) {
    let &View { mut x, mut y } = v;

    if x > 0 {
        x = -(x % TILE_WIDTH);
    } else {
        x = x.abs();
    }

    if y > 0 {
        y = -(y % TILE_HEIGHT);
    } else {
        y = y.abs();
    }

    (x, y)
}

fn calculate_color(t: &world::tile::Tile) -> [f32; 3] {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    match t.ground {
        1 => r = 1.0,
        _ => (),
    }

    if let Some(ref effects) = t.effects {
        for e in effects {
            match e {
                &world::tile::Effect::Lit(lum) => {
                    let c = (255.0 * lum).min(255.0) / 255.0;
                    r += c;
                    g += c;
                    b += c;
                },

                &world::tile::Effect::Marked(_) => {
                    r = 1.0;
                    g = 1.0;
                    b = 1.0;
                }
            }
        }
    }

    [r.min(1.0), g.min(1.0), b.min(1.0)]
}
