pub(crate) fn minimum_angle_difference(angle1: f32, angle2: f32) -> f32 {
    let phi = (angle1 - angle2).abs() % 360.0;
    if phi > 180.0 {
        360.0 - phi
    } else {
        phi
    }
}

pub struct Satellite {
    pub pseudo_random_noise: u16,
    pub elevation: u8,
    pub azimuth: u16,
    pub carrier_to_noise_density: u8,
}
