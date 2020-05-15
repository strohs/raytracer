pub mod lambertian;
pub use lambertian::*;

pub mod metal;
pub use metal::*;

use crate::common::{Ray, Color, Vec3};
use crate::common::hittable::HitRecord;

/// holds the details on how a `Material` scattered an incoming `Ray`.
/// `attenuation` field what `Color` was applied by the material to the incoming Ray
/// `scattered` field contains the new `Ray` that was scattered
#[derive(Default, Debug, Copy, Clone)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl ScatterRecord {
    pub fn new(attenuation: Color, scattered: Ray) -> Self {
        Self { attenuation, scattered }
    }
}

pub trait Material {
    /// returns `Some(ScatterRecord)` if this material scattered `r_in`. If this material did not
    /// scatter `r_in`, `None` is returned
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

/// returns a reflected `Vec3` between `v` and `n`, where `n` is a unit vector
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * (2.0 * v.dot(n))
}