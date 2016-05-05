use glium;
use cgmath;
use tex_atlas;

#[derive(Copy, Clone)]
struct Vertex {
    position: [u32; 2],
}

implement_vertex!(Vertex, position);

#[derive (Copy, Clone)]
struct Instance {
    screen_position: [u32; 2],
    tex_offset: [f32; 2],
    fg_color: [f32; 4],
    bg_color: [f32; 3],
}

implement_vertex!(Instance, screen_position, tex_offset, fg_color, bg_color);

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 1, 3, 2];
const QUAD: [Vertex; 4] = [
    Vertex { position: [0, 1], },
    Vertex { position: [1, 1], },
    Vertex { position: [0, 0], },
    Vertex { position: [1, 0], },
];

pub struct Tile {
    pub n: u32,
    pub fg_color: [f32; 4],
    pub bg_color: [f32; 3],

    /// Invisible tiles will only have a background color.
    pub visible: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            n: 0,
            fg_color: [1.0, 1.0, 1.0, 1.0],
            bg_color: [0.0, 0.0, 0.0],
            visible: true,
        }
    }
}

pub struct TileMap<'a> {
    /// size of the map (in tiles)
    size: (u32, u32),
    visible_tile_size: (u32, u32),

    /// state of the map
    tiles: Vec<Tile>,

    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,

    tex_atlas: &'a tex_atlas::TextureAtlas,
    tex_disable_smoothing: bool,
}

fn read_string(path: &str) -> String {
    use std::io::Read;
    use std::fs::File;

    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

impl<'a> TileMap<'a> {
    pub fn new<F>(display: &F, size: (u32, u32), visible_tile_size: (u32, u32), tex_atlas: &'a tex_atlas::TextureAtlas) -> Self

        where F: glium::backend::Facade {

        use glium::index::PrimitiveType;

        let cnt = size.0 * size.1;
        let tiles = (0..cnt).map(|_| Default::default()).collect();

        let vertices = glium::VertexBuffer::new(display, &QUAD).unwrap();
        let indices = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &QUAD_INDICES).unwrap();

        let vertex_shader = read_string("assets/tile_map.vert");
        let fragment_shader = read_string("assets/tile_map.frag");
        let program = glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap();

        let tex_disable_smoothing = visible_tile_size != tex_atlas.tile_size();

        TileMap {
            size: size,
            visible_tile_size: visible_tile_size,
            tiles: tiles,
            vertices: vertices,
            indices: indices,
            program: program,
            tex_atlas: tex_atlas,
            tex_disable_smoothing: tex_disable_smoothing,
        }
    }

    pub fn set_tile(&mut self, x: u32, y: u32, t: Tile) {
        let idx = (self.size.0 * y + x) as usize;
        self.tiles[idx] = t;
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    fn create_instances<F>(&self, display: &F) -> glium::VertexBuffer<Instance>
        where F: glium::backend::Facade {

        let (mw, _) = self.size;
        let (tw, th) = self.visible_tile_size;
        let (ac, _) = self.tex_atlas.tile_count();
        let r = self.tex_atlas.ratio();

        let data = self.tiles.iter().zip(0..).map(|(t, i)| {
            // TODO remove division?
            let x = (i % mw) * tw;
            let y = (i / mw) * th;

            let tx = (t.n % ac) as f32 * r[0];
            let ty = (t.n / ac) as f32 * r[1];

            let fg = if t.visible {
                t.fg_color
            } else {
                [0.0, 0.0, 0.0, 0.0]
            };

            Instance { screen_position: [x, y], tex_offset: [tx, ty],
                       fg_color: fg, bg_color: t.bg_color }
        }).collect::<Vec<Instance>>();

        glium::VertexBuffer::dynamic(display, &data).unwrap()
    }
}

impl<'a> super::Renderable for TileMap<'a> {
    fn render<F, S>(&self, display: &F, target: &mut S, viewport: &super::Viewport)
        where F: glium::backend::Facade, S: glium::Surface {

        let (w, h) = (viewport.size.0 as f32, viewport.size.1 as f32);
        let proj: [[f32; 4]; 4] = cgmath::ortho(0.0, w, h, 0.0, -1.0, 1.0).into();

        let uniforms = uniform! {
            matrix: proj,
            tile_size: self.visible_tile_size,
            tex: {
                let s = self.tex_atlas.texture().sampled();

                // TODO find better solution for pixel-perfect tiles
                if self.tex_disable_smoothing {
                    s.wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
                }

                s
            },
            tex_ratio: self.tex_atlas.ratio(),
        };

        let instances = self.create_instances(display);

        // TODO move to arguments?
        let params = glium::DrawParameters {
            viewport: {
                let (x, y) = viewport.position;
                let (w, h) = viewport.size;
                Some(glium::Rect { left: x, bottom: y, width: w, height: h })
            },
            .. Default::default()
        };

        target.draw((&self.vertices, instances.per_instance().unwrap()),
                    &self.indices,
                    &self.program,
                    &uniforms,
                    &params).unwrap();
    }
}
