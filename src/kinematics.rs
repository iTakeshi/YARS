pub fn forward(
        position_offset: &[f32; 3],
        angle_offset: &f32,
        length: &f32,
        angle: &f32,
        ) -> ([f32; 3], f32) {
    (
        [
        position_offset[0] + length * (angle_offset + angle).cos(),
        position_offset[1] + length * (angle_offset + angle).sin(),
        position_offset[2],
        ],
        angle_offset + angle,
        )
}
