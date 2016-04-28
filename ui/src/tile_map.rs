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

const QUAD_INDICES: &'static [u16] = &[0u16, 1, 2, 1, 3, 2];
const QUAD: &'static [Vertex] = &[
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

pub struct TileMap {
    /// size of the individual tile (in px)
    tile_size: (u32, u32),

    /// size of the map (in tiles)
    map_size: (u32, u32),

    /// size of the viewport (in px)
    view_size: (u32, u32),

    /// state of the map
    tiles: Vec<Tile>,

    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,

    tex_atlas: tex_atlas::TextureAtlas,
}

impl TileMap {
    pub fn new<F>(display: &F, tile_size: (u32, u32), map_size: (u32, u32), view_size: (u32, u32),
                  tex_atlas: tex_atlas::TextureAtlas) -> Self
        where F: glium::backend::Facade {

        use glium::index::PrimitiveType;

        let cnt = map_size.0 * map_size.1;
        let tiles = (0..cnt).map(|_| Default::default()).collect();

        let vertices = glium::VertexBuffer::new(display, &QUAD).unwrap();
        let indices = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, QUAD_INDICES).unwrap();

        let vertex_shader = include_str!("shaders/tile_map.vert");
        let fragment_shader = include_str!("shaders/tile_map.frag");
        let program = glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap();

        TileMap {
            tile_size: tile_size,
            map_size: map_size,
            view_size: view_size,
            tiles: tiles,
            vertices: vertices,
            indices: indices,
            program: program,
            tex_atlas: tex_atlas,
        }
    }

    pub fn set_tile(&mut self, x: u32, y: u32, t: Tile) {
        let idx = (self.map_size.0 * y + x) as usize;
        self.tiles[idx] = t;
    }

    pub fn map_size(&self) -> (u32, u32) {
        self.map_size
    }

    fn create_instances<F>(&self, display: &F) -> glium::VertexBuffer<Instance>
        where F: glium::backend::Facade {

        let (tw, th) = self.tile_size;
        let (mw, _) = self.map_size;
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

pub trait Renderable {
    fn render<F, S>(&self, display: &F, target: &mut S) where F: glium::backend::Facade, S: glium::Surface;
}

impl Renderable for TileMap {
    fn render<F, S>(&self, display: &F, target: &mut S)
        where F: glium::backend::Facade, S: glium::Surface {

        let (w, h) = (self.view_size.0 as f32, self.view_size.1 as f32);
        let proj: [[f32; 4]; 4] = cgmath::ortho(0.0, w, h, 0.0, -1.0, 1.0).into();

        let uniforms = uniform! {
            matrix: proj,
            tile_size: self.tile_size,
            tex: self.tex_atlas.texture(),
            tex_ratio: self.tex_atlas.ratio(),
        };

        let instances = self.create_instances(display);

        target.draw((&self.vertices, instances.per_instance().unwrap()),
                    &self.indices,
                    &self.program,
                    &uniforms,
                    &Default::default()).unwrap();
    }
}
