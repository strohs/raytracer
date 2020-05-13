pub mod vec3;
pub use vec3::*;

pub mod rgb;
pub use rgb::*;

pub mod ray;
pub use ray::*;

/// alias for a 3D point with x,y,z coordinates
pub type Point3 = Vec3;

/// alias for a RGB color with three color components
pub type Color = Vec3;