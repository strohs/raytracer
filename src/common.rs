pub mod vec3;
pub use vec3::*;

pub mod ray;
pub use ray::*;

pub mod camera;
pub use camera::*;

pub mod color;
pub use color::Color;

/// alias for a 3D point with x,y,z coordinates
pub type Point3 = Vec3;




/// utility function for converting degrees to radians
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * core::f64::consts::PI / 180.0
}


/// clamps `x` to the range `[min..=max]`
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        _xmin if x < min => min,
        _xmax if x > max => max,
        _ => x,
    }
}