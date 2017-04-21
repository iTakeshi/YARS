#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};

extern crate nalgebra as na;
use na::Vector2;

extern crate obj;

mod dynamics;
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

    let mut torque = Vector2::new(0.0, 0.0f32);
    let mut angular_velocity = Vector2::new(0.0, 0.0f32);
    let mut angle = Vector2::new(0.0, 0.0f32);
    let length = Vector2::new(0.1, 0.1f32);

    let mut position_tip = [
        [0.0, 0.0, 0.0f32],
        [10.0, 0.0, 0.0f32],
        [20.0, 0.0, 0.0f32],
    ];
    let mut rotation = [
        [0.0, 0.0, 0.0f32],
        [0.0, 0.0, 0.0f32],
        [0.0, 0.0, 0.0f32],
    ];

    'mainloop: loop {
        for ev in display.poll_events() {
            use glium::glutin::{Event, VirtualKeyCode as VKC};
            use glium::glutin::ElementState::{Pressed, Released};
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(Pressed, _, Some(VKC::Q)) => return,
                Event::KeyboardInput(Pressed, _, Some(VKC::U)) => torque[0] =  1.5,
                Event::KeyboardInput(Pressed, _, Some(VKC::I)) => torque[0] = -1.5,
                Event::KeyboardInput(Pressed, _, Some(VKC::J)) => torque[1] =  1.5,
                Event::KeyboardInput(Pressed, _, Some(VKC::K)) => torque[1] = -1.5,
                Event::KeyboardInput(Released, _, Some(VKC::U)) => torque[0] = 0.0,
                Event::KeyboardInput(Released, _, Some(VKC::I)) => torque[0] = 0.0,
                Event::KeyboardInput(Released, _, Some(VKC::J)) => torque[1] = 0.0,
                Event::KeyboardInput(Released, _, Some(VKC::K)) => torque[1] = 0.0,
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

        match dynamics::lagrange(&torque, angular_velocity, angle, &length) {
            (v, a) => {
                angular_velocity = v;
                angle = a;
            }
        }

        for n in 1..position_tip.len() {
            match kinematics::forward(
                    &position_tip[n - 1],
                    &rotation[n - 1][2],
                    &10.0f32,
                    &angle[n - 1],
                    ) {
                (t, r) => {
                    position_tip[n] = t;
                    rotation[n][2] = r;
                }
            };
            let model_matrix = matrix::model_matrix(&position_tip[n - 1], &rotation[n]);
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
