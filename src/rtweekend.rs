pub const infinity: f64 = f64::INFINITY;
pub const pi: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * pi / 180.0;
}
