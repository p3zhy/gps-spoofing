pub(crate) fn minimum_angle_difference(angle1: f32, angle2: f32) -> f32 {
    let phi = (angle1 - angle2).abs() % 360.0;
    if phi > 180.0 {
        360.0 - phi
    } else {
        phi
    }
}

pub struct UnixTimestamp(u64);

impl UnixTimestamp {
    pub fn new(timestamp: u64) -> Self {
        UnixTimestamp(timestamp)
    }
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}
