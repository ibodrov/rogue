#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate time;

fn main() {
    use glium::{DisplayBuild, Surface};
    use glium::glutin::{Event, VirtualKeyCode};

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [i32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&display, &[
            Vertex { position: [0,  16], color: [1.0, 0.0, 0.0], },
            Vertex { position: [16, 16], color: [0.0, 1.0, 0.0], },
            Vertex { position: [0,  0],  color: [0.0, 0.0, 1.0], },
            Vertex { position: [16, 0],  color: [1.0, 0.0, 0.0], },
        ]).unwrap()
    };

    let index_buffer = glium::IndexBuffer::new(&display,
                                               glium::index::PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2, 1, 3, 2]).unwrap();

    let per_instance = {
        #[derive (Copy, Clone)]
        struct Attr {
            screen_position: (i32, i32),
            visibility: f32,
        }

        implement_vertex!(Attr, screen_position, visibility);

        let data = {
            let mut v = Vec::new();

            for i in 0..100 {
                for j in 0..100 {
                    let x: i32 = i * 16;
                    let y: i32 = j * 16;
                    let vis = if i < 10 && j < 10 { 1.0 } else { 0.0 };

                    v.push(Attr { screen_position: (x, y), visibility: vis });
                }
            }

            v
        };

        glium::vertex::VertexBuffer::dynamic(&display, &data).unwrap()
    };

    let vertex_shader = r#"
        #version 150

        in ivec2 position;
        in ivec2 screen_position;
        in vec3 color;
        in float visibility;

        uniform mat4 matrix;
        out vec3 v_Color;
        out float v_Visibility;

        void main() {
            gl_Position = matrix * vec4(position + screen_position, 0.0, 1.0);
            v_Color = color;
            v_Visibility = visibility;
        }
    "#;

    let fragment_shader = r#"
        #version 150

        in vec3 v_Color;
        in float v_Visibility;
        out vec4 color;

        void main() {
            if (v_Visibility == 0.0) {
                discard;
            }

            color = vec4(v_Color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();
    let proj: [[f32; 4]; 4] = cgmath::ortho(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0).into();

    let mut t0 = time::precise_time_s();
    let mut frames = 0;

    loop {
        for ev in display.poll_events() {
            match ev {
                Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return,
                _ => (),
            }
        }

        let uniforms = uniform! {
            matrix: proj,
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw((&vertex_buffer, per_instance.per_instance().unwrap()), &index_buffer, &program, &uniforms, &Default::default()).unwrap();
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
