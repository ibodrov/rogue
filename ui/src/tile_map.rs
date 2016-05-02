extern crate glium;
extern crate cgmath;
extern crate tex_atlas;

#[derive(Copy, Clone)]
struct Vertex {
    position: [u32; 2],
}

implement_vertex!(Vertex, position);

#[derive (Copy, Clone)]
struct Instance {
    screen_position: [u32; 2],
    tex_offset: [f32; 2],
}

implement_vertex!(Instance, screen_position, tex_offset);

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 1, 3, 2];
const QUAD: [Vertex; 4] = [
    Vertex { position: [0, 1], },
    Vertex { position: [1, 1], },
    Vertex { position: [0, 0], },
    Vertex { position: [1, 0], },
];

pub struct Tile(pub u32);

impl Default for Tile {
    fn default() -> Self {
        Tile(0)
    }
}

pub struct TileMap<'a> {
    /// size of the map (in tiles)
    size: (u32, u32),

    /// state of the map
    tiles: Vec<Tile>,

    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,

    tex_atlas: &'a tex_atlas::TextureAtlas,
}

impl<'a> TileMap<'a> {
    pub fn new<F>(display: &F, size: (u32, u32), tex_atlas: &'a tex_atlas::TextureAtlas) -> Self

        where F: glium::backend::Facade {

        use glium::index::PrimitiveType;

        let cnt = size.0 * size.1;
        let tiles = (0..cnt).map(|_| Default::default()).collect();

        let vertices = glium::VertexBuffer::new(display, &QUAD).unwrap();
        let indices = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &QUAD_INDICES).unwrap();

        let vertex_shader = include_str!("shaders/tile_map.vert");
        let fragment_shader = include_str!("shaders/tile_map.frag");
        let program = glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap();

        TileMap {
            size: size,
            tiles: tiles,
            vertices: vertices,
            indices: indices,
            program: program,
            tex_atlas: tex_atlas,
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
        let (tw, th) = self.tex_atlas.tile_size();
        let (ac, _) = self.tex_atlas.tile_count();
        let r = self.tex_atlas.ratio();

        let idx = 0..;
        let data = self.tiles.iter().zip(idx).map(|(t, i)| {
            // TODO remove division?
            let x = (i % mw) * tw;
            let y = (i / mw) * th;

            let tx = (t.0 % ac) as f32 * r[0];
            let ty = (t.0 / ac) as f32 * r[1];

            Instance { screen_position: [x, y], tex_offset: [tx, ty] }
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
            tile_size: self.tex_atlas.tile_size(),
            tex: {
                use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
                let s = self.tex_atlas.texture().sampled();
                s.magnify_filter(MagnifySamplerFilter::Linear);
                s.minify_filter(MinifySamplerFilter::Linear);
                s
            },
            tex_ratio: self.tex_atlas.ratio(),
        };

        let instances = self.create_instances(display);

        // TODO move to arguments?
        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
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
