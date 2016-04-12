#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;
extern crate time;

mod world_render;

use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Rogue".to_string())
        .with_dimensions(1024, 768);

    let (window, mut device, mut factory, main_color, _) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let wr = world_render::RenderableWorld::new(&mut factory, main_color);

    let mut t1 = 0.0;
    let mut frames = 0;

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        wr.render(&mut encoder);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();

        frames += 1;

        let t2 = time::precise_time_s();
        if t2 - t1 > 1.0 {
            t1 = t2;
            window.set_title(&format!("FPS: {}", frames));
            frames = 0;
        }
    }
}
