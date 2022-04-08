pub mod lambertian;
pub use lambertian::*;

pub mod metal;
pub use metal::*;

pub mod dielectric;
pub use dielectric::*;

pub mod diffuse_light;
pub use diffuse_light::*;

pub mod isotropic;
pub use isotropic::*;

use crate::common::{Color, Point3, Ray, Vec3};
use crate::hittable::HitRecord;
use std::fmt::Debug;
use std::ops::Neg;

/// holds the results of how a `Material` scattered an incoming `Ray`.
/// `attenuation` contains what `Color` was applied by the material to the incoming Ray
/// `scattered` contains the new `Ray` that was scattered
#[derive(Default, Debug, Copy, Clone)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl ScatterRecord {
    pub fn new(attenuation: Color, scattered: Ray) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}

/// A trait for different material types that could be applied to a `Hittable`.
pub trait Material: Send + Sync + Debug {
    /// Returns `Some(ScatterRecord)` if this material scattered the incoming Ray `r_in`.
    /// If this material did not scatter `r_in`, `None` is returned
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;

    /// Returns a `Color` emitted by this material. The base implementation of this trait
    /// returns black as the default color
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

/// Returns a *reflected* `Vec3` between `v` and `n`, where `n` is a unit vector
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * (2.0 * v.dot(n))
}

/// uses Snell's law to return the direction of a Ray hitting a refractive material
/// `uv` is the incoming ray direction as a unit vector
/// `n` is the normal vector of the point that was hit on the hittable
/// `etai_over_etat` is the refractive index of the material
fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = uv.neg().dot(n);
    let r_out_parallel = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_perp = -1.0 * (1.0 - r_out_parallel.length_squared()).sqrt() * *n;
    r_out_parallel + r_out_perp
}

/// Schlick's approximation for determining how much light is **reflected** for a glass material
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
