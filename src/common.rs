pub mod vec3;
pub use vec3::*;

pub mod rgb;
pub use rgb::*;

pub mod ray;
pub use ray::*;

pub mod hittable;

pub mod sphere;
pub use sphere::*;

/// alias for a 3D point with x,y,z coordinates
pub type Point3 = Vec3;

/// alias for a RGB color with three color components
pub type Color = Vec3;

use core::f64::consts::PI;
// infinity is f64::INFINITY

/// utility function for converting degrees to radians
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}