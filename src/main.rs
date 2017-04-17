#![feature(const_fn)]

#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};

extern crate nalgebra as na;

extern crate obj;

mod matrix;
mod program;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24).build_glium().unwrap();
    let axis_program = program::axis_program(&display);
    let axis_vertices = program::axis_vertices(&display);
    let model_program = program::model_program(&display);
    let (model_vertices, model_indices) = program::model_buffers(&display);
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    let mut translation = [0.0, 0.0, 0.0f32];
    let mut rotation = [0.0, 0.0, 0.0f32];

    'mainloop: loop {
        for ev in display.poll_events() {
            use glium::glutin::{ElementState, Event, VirtualKeyCode};
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(
                    ElementState::Pressed, _, Some(VirtualKeyCode::Q)) => return,
                _ => (),
            }
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.3, 0.3, 1.0, 1.0), 1.0);

        let view_perspective_matrix =
            matrix::view_perspective_matrix(target.get_dimensions());
        let model_matrix = matrix::model_matrix(&translation, &rotation);

        target.draw(
            &axis_vertices,
            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            &axis_program,
            &uniform! {
                rgb: [1.0, 0.0, 0.0f32],
                view_perspective: view_perspective_matrix,
            },
            &params,
            ).unwrap();

        target.draw(
            &model_vertices,
            &model_indices,
            &model_program,
            &uniform! {
                light: [-1.0, 0.4, 0.9f32],
                view_perspective: view_perspective_matrix,
                model: model_matrix,
            },
            &params
            ).unwrap();

        target.finish().unwrap();
    }
}
