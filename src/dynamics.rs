use na::{Matrix2, Vector2};

pub fn lagrange(
        torque: &Vector2<f32>,
        angular_velocity: Vector2<f32>,
        angle: Vector2<f32>,
        length: &Vector2<f32>,
        ) -> (Vector2<f32>, Vector2<f32>){

    let gravity = 9.80665f32;
    let mass = Vector2::new(1.0, 1.0f32);
    let moment = Vector2::new(0.003, 0.003f32);

    let m = Matrix2::new(
        mass[0] * (length[0] / 2.0).powi(2) + mass[1] * length[0].powi(2) + mass[1] * (length[1] / 2.0).powi(2)
            + moment[0] + moment[1] + 2.0 * mass[1] * length[0] * (length[1] / 2.0) * angle[1].cos(),
        mass[1] * (length[1] / 2.0).powi(2) + moment[1] + mass[1] * length[0] * (length[1] / 2.0) * angle[1].cos(),
        mass[1] * (length[1] / 2.0).powi(2) + moment[1] + mass[1] * length[0] * (length[1] / 2.0) * angle[1].cos(),
        mass[1] * (length[1] / 2.0).powi(2) + moment[1],
        );

    let h = Vector2::new(
        -(mass[1] * length[0] * (length[1] / 2.0) * (2.0 * angular_velocity[0] + angular_velocity[1]) * angular_velocity[1] * angle[1].sin()),
        mass[1] * length[0] * (length[1] / 2.0) * angular_velocity[0].powi(2) * angle[1].sin(),
        );

    let g = Vector2::new(
        (mass[0] * gravity * (length[0] / 2.0) + mass[1] * gravity * length[0]) * angle[0].cos()
            + mass[1] * gravity * (length[1] / 2.0) * (angle[0] + angle[1]).cos(),
        mass[1] * gravity * (length[1] / 2.0) * (angle[0] + angle[1]).cos(),
        );

    let a = m.try_inverse().unwrap() * (torque - h - g);

    let frame = 0.006;
    (
        angular_velocity + (a - 2.0 * angular_velocity) * frame,
        angle + angular_velocity * frame,
        )
}
