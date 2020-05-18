use crate::common::{Color, Ray, Vec3};
use crate::material::{Material, ScatterRecord};
use crate::hittable::HitRecord;

/// lambertian diffuse material
#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    // the proportion of the incident light that is reflected by this material
    pub albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {

    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        Some(
            ScatterRecord {
                scattered: Ray::new(rec.p, scatter_direction),
                attenuation: self.albedo,
            }
        )
    }
}