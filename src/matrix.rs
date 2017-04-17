use na::{
    Isometry3,
    Matrix4,
    Perspective3,
    Point3,
    Rotation3,
    Translation3,
    Vector3,
};

pub fn view_perspective_matrix(window_size: (u32, u32)) -> [[f32; 4]; 4] {
    let angle_of_view = 1.0f32;
    let z_far = 1000.0f32;
    let z_near = 0.1f32;

    let camera_position = [15.0, 8.0, 21.0f32];
    let camera_target   = [5.0, 12.0, -5.0f32];
    let camera_up       = [0.0, 1.0, 0.0f32];

    let view_perspective = {
        let perspective = {
            let aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
            Perspective3::new(aspect_ratio, angle_of_view, z_near, z_far).unwrap()
        };
        let view = Isometry3::look_at_rh(
            &Point3::new(camera_position[0], camera_position[1], camera_position[2]),
            &Point3::new(camera_target[0], camera_target[1], camera_target[2]),
            &Vector3::new(camera_up[0], camera_up[1], camera_up[2]),
            ).to_homogeneous();
        perspective * view
    };

    unsafe {
        *(view_perspective.as_slice().as_ptr() as *const [[f32; 4]; 4])
    }
}

pub fn model_matrix(translation: &[f32; 3], rotation: &[f32; 3]) -> [[f32; 4]; 4] {
    let model = {
        let translation = Translation3::new(
            translation[0], translation[1], translation[2]).to_homogeneous();
        let rotation = Rotation3::from_euler_angles(
            rotation[0], rotation[1], rotation[2]).to_homogeneous();
        let scale = Matrix4::new_scaling(1.0f32);
        translation * rotation * scale
    };

    unsafe {
        *(model.as_slice().as_ptr() as *const [[f32; 4]; 4])
    }
}
