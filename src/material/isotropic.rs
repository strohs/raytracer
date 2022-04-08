use crate::common::{Ray, Vec3};
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::texture::Texture;
use std::sync::Arc;

/// An `Isotropic` material has properties that are identical in all directions
#[derive(Debug)]
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    /// Returns a new Isotropic material with the specified albedo texture
    pub fn from(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    /// The `scatter` function of an isotropic picks a uniform random direction
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some(ScatterRecord::new(attenuation, scattered))
    }
}
