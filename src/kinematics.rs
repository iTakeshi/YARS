pub fn forward(
        base: &[f32; 3],
        angle_offset: &f32,
        length: &f32,
        angle: &f32,
        ) -> ([f32; 3], f32) {
    (
        [
            base[0] + length * (angle_offset + angle).cos(),
            base[1] + length * (angle_offset + angle).sin(),
            base[2],
        ],
        angle_offset + angle,
        )
}
