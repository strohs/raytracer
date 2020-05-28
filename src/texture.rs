pub mod solid;
pub use solid::*;

pub mod checker;
pub use checker::*;

use crate::common::{Color, Point3, Vec3};
use std::f64::consts::PI;


/// Computes the u,v coordinates for a sphere given its center point.
/// `p` is the center point of a unit sphere centered at the origin.
///  Returns a tuple `(u,v)`, containing the sphere's u,c coordinate
pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    let u = 1.0 - (phi + PI) / 2.0 * PI;
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}


/// A trait for dealing with textures. A texture in graphics is usually a function that makes
/// the colors on a surface procedural. This procedure can be synthesis code, or it could be an
/// image lookup, or a combination of both.
///
pub trait Texture: Send + Sync + std::fmt::Debug {

    /// Returns the color of a texture at the given `u,v` coordinate and point `p`
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}