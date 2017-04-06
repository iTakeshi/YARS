#[macro_use]
extern crate glium;

extern crate nalgebra as na;
use na::{Isometry3, Matrix4, Perspective3, Point3, Rotation3, Translation3, Vector3};
use na::storage::Storage;

mod teapot;
mod vertex;
use vertex::Vertex;

fn read_file(path: &str) -> String {
    use std::error::Error;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new(path);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        Ok(_) => s,
    }
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();

    let axis_vert_shader_src = read_file("shaders/axis.vert");
    let axis_frag_shader_src = read_file("shaders/axis.frag");
    let axis_program = glium::Program::from_source(&display, &*axis_vert_shader_src, &*axis_frag_shader_src, None).unwrap();
    let axis_vertices = glium::VertexBuffer::new(&display, &[
        Vertex { position: (-1000.0,     0.0,     0.0) },
        Vertex { position: ( 1000.0,     0.0,     0.0) },
        Vertex { position: (    0.0, -1000.0,     0.0) },
        Vertex { position: (    0.0,  1000.0,     0.0) },
        Vertex { position: (    0.0,     0.0, -1000.0) },
        Vertex { position: (    0.0,     0.0,  1000.0) },
    ]).unwrap();

    let model_vert_shader_src = read_file("shaders/model.vert");
    let model_frag_shader_src = read_file("shaders/model.frag");
    let model_program = glium::Program::from_source(&display, &*model_vert_shader_src, &*model_frag_shader_src, None).unwrap();
    let model_vertices = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let model_normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let model_indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    let mut angle_of_view = 1.0f32;
    let mut z_far = 1000.0f32;
    let mut z_near = 0.1f32;

    let mut camera_position = [2.0, 5.0, 30.0f32];
    let mut camera_target   = [0.0, 0.0,  0.0f32];
    let mut camera_up       = [0.0, 1.0,  0.0f32];

    let mut model_x = 0.0f32;
    let mut model_y = 0.0f32;
    let mut model_z = 0.0f32;
    let mut model_pitch = 0.0f32;
    let mut model_yaw = 0.0f32;
    let mut model_roll = 0.0f32;

    loop {
        for ev in display.poll_events() {
            use glium::glutin::{ElementState, Event, VirtualKeyCode};
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                    match code {
                        VirtualKeyCode::Key1 => angle_of_view += 0.1,
                        VirtualKeyCode::Key0 => angle_of_view -= 0.1,
                        VirtualKeyCode::S => model_z -= 1.0,
                        VirtualKeyCode::W => model_z += 1.0,
                        VirtualKeyCode::A => model_x -= 1.0,
                        VirtualKeyCode::D => model_x += 1.0,
                        VirtualKeyCode::F => model_y -= 1.0,
                        VirtualKeyCode::R => model_y += 1.0,
                        VirtualKeyCode::Q => model_pitch -= 0.1,
                        VirtualKeyCode::E => model_pitch += 0.1,
                        VirtualKeyCode::Z => model_roll -= 0.1,
                        VirtualKeyCode::V => model_roll += 0.1,
                        VirtualKeyCode::X => model_yaw -= 0.1,
                        VirtualKeyCode::C => model_yaw += 0.1,
                        _ => (),
                    }
                },
                _ => ()
            }
        }

        let mut target = display.draw();

        let light = [-1.0, 0.4, 0.9f32];
        let view_perspective = {
            let perspective = {
                let (width, height) = target.get_dimensions();
                let aspect_ratio = width as f32 / height as f32;
                Perspective3::new(aspect_ratio, angle_of_view, z_near, z_far).unwrap()
            };
            let view = Isometry3::look_at_rh(
                &Point3::new(camera_position[0], camera_position[1], camera_position[2]),
                &Point3::new(camera_target[0], camera_target[1], camera_target[2]),
                &Vector3::new(camera_up[0], camera_up[1], camera_up[2]),
                ).to_homogeneous();
            perspective * view
        };
        let model = {
            let translation = Translation3::new(model_x, model_y, model_z).to_homogeneous();
            let rotation = Rotation3::from_euler_angles(model_roll, model_pitch, model_yaw).to_homogeneous();
            let scale = Matrix4::new_scaling(0.01f32);
            translation * rotation * scale
        };
        let view_perspective_array = unsafe {
            *(view_perspective.as_slice().as_ptr() as *const [[f32; 4]; 4])
        };
        let model_array = unsafe {
            *(model.as_slice().as_ptr() as *const [[f32; 4]; 4])
        };

        target.clear_color_and_depth((0.3, 0.3, 1.0, 1.0), 1.0);
        target.draw(
            &axis_vertices,
            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            &axis_program,
            &uniform! {
                rgb: [1.0, 0.0, 0.0f32],
                view_perspective: view_perspective_array,
            },
            &params,
            ).unwrap();
        target.draw(
            (&model_vertices, &model_normals),
            &model_indices,
            &model_program,
            &uniform! {
                light: light,
                view_perspective: view_perspective_array,
                model: model_array,
            },
            &params
            ).unwrap();
        target.finish().unwrap();
    }
}
