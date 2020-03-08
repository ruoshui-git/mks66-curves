use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn create_file(filepath: &str) -> BufWriter<File> {
    let path = Path::new(filepath);
    let display = path.display();
    match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(file) => BufWriter::new(file),
    }
}

pub fn polar_to_xy(mag: f64, angle_degrees: f64) -> (f64, f64) {
    let (dy, dx) = angle_degrees.to_radians().sin_cos();
    (dx * mag, dy * mag)
}

/// Returns the coefficients of the cubic bezier curve (a, b, c, d)
/// # Arguments
/// `p0`, `p1`, `p2`, `p3` - x or y component of control points
pub fn compute_bezier3_coef(p0: f64, p1: f64, p2: f64, p3: f64) -> (f64, f64, f64, f64) {
    // (-P0 + 3P1 - 3P2 + P3)t^3 + (3P0 - 6P1 + 3P2)t^2 + (-3P0 + 3P1)t + P0
    // == at^3 + b^2 + ct + d
    (
        -p0 + 3.0 * (p1 - p2) + p3,
        3.0 * p0 - 6.0 * p1 + 3.0 * p2,
        3.0 * (-p0 + p1),
        p0,
    )
}

/// Returns the coefficients of the cubic hermite curve (a, b, c, d)
/// # Arguments
/// `p0`, `p1` - x or y component of endpoints
/// 
/// `r0`, `r1` - x or y rate of change at each endpoint
pub fn compute_hermite3_coef(p0: f64, p1: f64, r0: f64, r1: f64) -> (f64, f64, f64, f64) {
    (
        2.0 * (p0 - p1) + r0 + r1,
        3.0 * (-p0 + p1) - 2.0 * r0 - r1,
        r0,
        p0,
    )
}
