use na::{Matrix2, Vector2};

pub fn lagrange(
        torque: &Vector2<f32>,
        angular_velocity: Vector2<f32>,
        angle: Vector2<f32>,
        length: &Vector2<f32>,
        step: &f32,
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

    let acc = m.try_inverse().unwrap() * (torque - h - g - 0.1 * angular_velocity);
    let vel = angular_velocity + acc * *step;
    let ang = angle + vel * *step;

    print!("{}, {}, {}, {}, {}, {}, {}, {}, ",
             acc[0], acc[1], vel[0], vel[1], ang[0], ang[1], torque[0], torque[1]);

    (vel, ang)
}

pub fn identified(
        torque: &Vector2<f32>,
        angular_velocity: Vector2<f32>,
        angle: Vector2<f32>,
        length: &Vector2<f32>,
        step: &f32,
        ) -> (Vector2<f32>, Vector2<f32>){

    //let (m1, m2, r, g1, g2) = (0.02071123, 0.00542086, 0.00492300, 1.45452541, 0.48267614);
    let (m1, m2, r, g1, g2, b1, b2) = (
        0.01616451,
        0.00368036,
        0.00389049,
        1.33251507,
        0.28745762,
        0.04628561,
        0.00737733,
        );

    let m = Matrix2::new(
        m1 + 2.0 * r * angle[1].cos(),
        m2 + r * angle[1].cos(),
        m2 + r * angle[1].cos(),
        m2,
        );

    let f = torque -
        Vector2::new(
            -r * (2.0 * angular_velocity[0] + angular_velocity[1]) * angular_velocity[1] * angle[1].sin(),
            r * angular_velocity[0].powi(2) * angle[1].sin(),
            ) -
        Vector2::new(
            g1 * angle[0].cos() + g2 * (angle[0] + angle[1]).cos(),
            g2 * (angle[0] + angle[1]).cos(),
            ) -
        Vector2::new(
            b1 * angular_velocity[0],
            b2 * angular_velocity[1],
            );

    let acc = m.try_inverse().unwrap() * f;
    let vel = angular_velocity + acc * *step;
    let ang = angle + vel * *step;

    (vel, ang)
}
