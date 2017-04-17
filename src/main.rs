#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};

extern crate nalgebra as na;

extern crate obj;

mod kinematics;
mod matrix;
mod program;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24).build_glium().unwrap();
    let axis_program = program::axis_program(&display);
    let axes = program::axes(&display);
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

    let length = [10.0, 10.0f32];
    let mut angle = [0.0, 0.0f32];

    let mut translation = [
        [0.0, 0.0, 0.0f32],
        [10.0, 0.0, 0.0f32],
    ];
    let mut rotation = [
        [0.0, 0.0, 0.0f32],
        [0.0, 0.0, 0.0f32],
    ];

    'mainloop: loop {
        for ev in display.poll_events() {
            use glium::glutin::{Event, VirtualKeyCode as VKC};
            use glium::glutin::ElementState::Pressed;
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(Pressed, _, Some(VKC::Q)) => return,
                Event::KeyboardInput(Pressed, _, Some(VKC::U)) => angle[0] += 0.1,
                Event::KeyboardInput(Pressed, _, Some(VKC::I)) => angle[0] -= 0.1,
                Event::KeyboardInput(Pressed, _, Some(VKC::J)) => angle[1] += 0.1,
                Event::KeyboardInput(Pressed, _, Some(VKC::K)) => angle[1] -= 0.1,
                _ => (),
            }
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.3, 0.3, 1.0, 1.0), 1.0);

        let view_perspective_matrix =
            matrix::view_perspective_matrix(target.get_dimensions());

        for axis in axes.iter() {
            target.draw(
                &axis.0,
                &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
                &axis_program,
                &uniform! {
                    rgb: axis.1,
                    view_perspective: view_perspective_matrix,
                },
                &params,
                ).unwrap();
        }

        for n in 0..translation.len() {
            let angle_offset = if n == 0 { 0.0f32 } else { rotation[n - 1][1] };
            match kinematics::forward(
                    &translation[n], &angle_offset, &length[n], &angle[n]) {
                (t, r) => {
                    if n < translation.len() - 1 {
                        translation[n + 1] = t;
                    }
                    rotation[n][1] = r;
                }
            };
            let model_matrix = matrix::model_matrix(&translation[n], &rotation[n]);
            target.draw(
                &model_vertices,
                &model_indices,
                &model_program,
                &uniform! {
                    light: [-1.0, 5.0, 5.0f32],
                    view_perspective: view_perspective_matrix,
                    model: model_matrix,
                },
                &params
                ).unwrap();
        }

        target.finish().unwrap();
    }
}
