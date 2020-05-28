use crate::common::{Ray, Vec3};
use crate::material::{Material, ScatterRecord};
use crate::hittable::HitRecord;
use crate::texture::Texture;
use std::sync::Arc;

/// lambertian diffuse material
#[derive(Debug)]
pub struct Lambertian {
    // the proportion of the incident light that is reflected by this material
    pub albedo: Arc<dyn Texture>
}

impl Lambertian {
    pub fn new(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some(
            ScatterRecord {
                scattered: Ray::new(rec.p, scatter_direction, r_in.time()),
                attenuation,
            }
        )
    }
}