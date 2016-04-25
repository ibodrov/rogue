use std;
use glium;
use cgmath;
use world;
use tex_atlas;

const TILE_WIDTH: i32 = 8;
const TILE_HEIGHT: i32 = 10;

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

pub struct View {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

const QUAD_INDICES: &'static [u16] = &[0u16, 1, 2, 1, 3, 2];
const QUAD: &'static [Vertex] = &[
    Vertex { position: [0, 1], },
    Vertex { position: [1, 1], },
    Vertex { position: [0, 0], },
    Vertex { position: [1, 0], },
];

pub struct WorldView {
    atlas: tex_atlas::TextureAtlas,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
    pub view: View,
}

impl WorldView {
    pub fn new<F: glium::backend::Facade>(display: &F) -> WorldView {
        let atlas = tex_atlas::load(display, std::path::Path::new("tiles.png")).unwrap();

        let vertex_buffer = glium::VertexBuffer::new(display, &QUAD).unwrap();

        let index_buffer = glium::IndexBuffer::new(display,
                                                   glium::index::PrimitiveType::TrianglesList,
                                                   QUAD_INDICES).unwrap();

        let vertex_shader = r#"
        #version 150

        in ivec2 position;
        in ivec2 screen_position;
        in vec3 color;

        uniform mat4 matrix;
        uniform ivec2 tile_size;
        uniform vec2 atlas_ratio;

        out vec3 v_Color;
        out vec2 v_TexCoords;

        void main() {
            gl_Position = matrix * vec4(position * tile_size + screen_position, 0.0, 1.0);

            float tile_offset_x = atlas_ratio.x * 2;
            float tile_offset_y = 0.0;
            float u = position.x * atlas_ratio.x + tile_offset_x;
            float v = 1.0 - (position.y * atlas_ratio.y + tile_offset_y);
            v_TexCoords = vec2(u, v);

            v_Color = color;
        }"#;

        let fragment_shader = r#"
        #version 150

        in vec3 v_Color;
        in vec2 v_TexCoords;

        uniform sampler2D tex;

        out vec4 color;

        void main() {
            color = texture(tex, v_TexCoords) * vec4(v_Color, 1.0);
        }"#;

        let program = glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap();

        let view = View { x: 0, y: 0, z: 0 };

        WorldView {
            atlas: atlas,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
            view: view
        }
    }

    pub fn render<F: glium::backend::Facade, S: glium::Surface>(&self, display: &F, target: &mut S, screen_size: (u32, u32), world: &world::World) {
        let proj: [[f32; 4]; 4] = cgmath::ortho(0.0, screen_size.0 as f32, screen_size.1 as f32, 0.0, -1.0, 1.0).into();

        let uniforms = uniform! {
            matrix: proj,
            tile_size: [TILE_WIDTH, TILE_HEIGHT],
            tex: self.atlas.texture(),
            atlas_ratio: self.atlas.ratio(),
        };

        let instances = {
            let data = {
                let view = &self.view;

                let world_view = {
                    let x = view.x / TILE_WIDTH;
                    let y = view.y / TILE_HEIGHT;
                    let z = view.z;

                    let sx = screen_size.0 / TILE_WIDTH as u32 + 1;
                    let sy = screen_size.1 / TILE_HEIGHT as u32 + 1;
                    let sz = 1;

                    world::render::View::new((x, y, z), (sx, sy, sz))
                };

                // smooth scrolling support
                let (view_dx, view_dy) = get_view_delta(&view);

                let render = world::render::RenderedView::render(&world, &world_view);
                let mut v = Vec::with_capacity(render.tiles_count() as usize);

                for (x, y, _, tile) in render.iter() {
                    let x = x as i32 * TILE_WIDTH + view_dx;
                    let y = y as i32 * TILE_HEIGHT + view_dy;

                    let color = calculate_color(&tile);
                    v.push(Instance { screen_position: [x, y], color: color });
                }

                v
            };

            glium::vertex::VertexBuffer::dynamic(display, &data).unwrap()
        };

        {
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            target.draw((&self.vertex_buffer, instances.per_instance().unwrap()),
                        &self.index_buffer,
                        &self.program,
                        &uniforms,
                        &Default::default()).unwrap();
        }
    }
}

fn get_view_delta(v: &View) -> (i32, i32) {
    let &View { mut x, mut y, .. } = v;

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
