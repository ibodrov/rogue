extern crate gfx;
extern crate gfx_device_gl;
extern crate cgmath;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_vertex_struct!(Vertex {
    pos: [i32; 2] = "a_Pos",
    color: [f32; 4] = "a_Color",
});

gfx_vertex_struct!(Instance {
    translate: [i32; 2] = "a_Translate",
});

gfx_pipeline!(pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
    offset: gfx::Global<[i32; 2]> = "u_Offset",
    out: gfx::RenderTarget<ColorFormat> = "Target0",
});

const TILE_SIZE: i32 = 16;

const QUAD: [Vertex; 4] = [
    Vertex { pos: [ 0,         TILE_SIZE ], color: [1.0, 0.0, 0.0, 1.0] },
    Vertex { pos: [ TILE_SIZE, TILE_SIZE ], color: [0.0, 1.0, 0.0, 1.0] },
    Vertex { pos: [ 0,         0 ], color: [0.0, 0.0, 1.0, 1.0] },
    Vertex { pos: [ TILE_SIZE, 0 ], color: [0.0, 0.0, 0.0, 1.0] },
];

const QUAD_INDEX: &'static [u16] = &[0, 1, 2, 1, 3, 2];
const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct RenderableWorld {
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    data: pipe::Data<gfx_device_gl::Resources>,
    slice: gfx::Slice<gfx_device_gl::Resources>,
}

struct View {
    x: i32,
    y: i32,
}

impl RenderableWorld {
    pub fn new(factory: &mut gfx_device_gl::Factory, out: gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>) -> Self {
        use gfx::traits::{Factory, FactoryExt};

        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/triangle_150.glslv"),
            include_bytes!("shader/triangle_150.glslf"),
            gfx::state::CullFace::Nothing,
            pipe::new()
        ).unwrap();

        let instance_cols = 1024 / TILE_SIZE * 8;
        let instance_rows = 768 / TILE_SIZE * 8;
        let instance_count = (instance_rows * instance_cols) as u32;

        let quad_instances = {
            let mut attributes = (0..instance_count).map(|_| Instance { translate: [0, 0] }).collect::<Vec<_>>();
            for i in 0..instance_rows {
                for j in 0..instance_cols {
                    let idx = (i * instance_cols + j) as usize;
                    let t = &mut attributes[idx].translate;
                    t[0] = j * TILE_SIZE;
                    t[1] = i * TILE_SIZE;
                }
            }
            factory.create_buffer_const(&attributes, gfx::BufferRole::Vertex, gfx::Bind::empty()).unwrap()
        };

        let (vertex_buffer, mut slice) = factory.create_vertex_buffer_indexed(&QUAD, QUAD_INDEX);
        slice.instances = Some((instance_count, 0));
        let proj = cgmath::ortho(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);

        let data = pipe::Data {
            vbuf: vertex_buffer,
            instance: quad_instances,
            transform: proj.into(),
            offset: [0, 0].into(),
            out: out,
        };

        RenderableWorld {
            pso: pso,
            data: data,
            slice: slice,
        }
    }

    pub fn render(&self, encoder: &mut gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>) {
        encoder.clear(&self.data.out, CLEAR_COLOR);
        encoder.draw(&self.slice, &self.pso, &self.data);
    }

    pub fn move_view(&mut self, dx: i32, dy: i32) {
        let o = &mut self.data.offset;
        o[0] += dx;
        o[1] += dy;
    }
}
