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
    pub n: u8,
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
    coords: Vec<[u32; 2]>,

    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,

    tex_atlas: &'a tex_atlas::TextureAtlas,
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

        let coords = {
            // cache all tile locations
            let (mw, _) = size;
            let (tw, th) = visible_tile_size;

            (0..cnt).map(|i| {
                let x = (i % mw) * tw;
                let y = (i / mw) * th;
                [x, y]
            }).collect()
        };

        let vertices = glium::VertexBuffer::immutable(display, &QUAD).unwrap();
        let indices = glium::IndexBuffer::immutable(display, PrimitiveType::TrianglesList, &QUAD_INDICES).unwrap();

        let vertex_shader = read_string("assets/tile_map.vert");
        let fragment_shader = read_string("assets/tile_map.frag");
        let program = glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap();

        TileMap {
            size: size,
            visible_tile_size: visible_tile_size,
            tiles: tiles,
            coords: coords,
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

        let tex_coords = self.tex_atlas.tex_coords();

        let data = self.tiles.iter()
            .zip(self.coords.iter())
            .map(|(tile, c)| {
                let (x, y) = (c[0], c[1]);

                let txc = tex_coords[tile.n as usize];
                let (tx, ty) = (txc[0], txc[1]);

                let fg = if tile.visible {
                    tile.fg_color
                } else {
                    [0.0, 0.0, 0.0, 0.0]
                };

                Instance { screen_position: [x, y], tex_offset: [tx, ty],
                           fg_color: fg, bg_color: tile.bg_color }
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
            tex: self.tex_atlas.texture().sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            tex_ratio: self.tex_atlas.tex_ratio(),
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
