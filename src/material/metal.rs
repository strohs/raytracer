use crate::common::{Color, Ray, Vec3};
use crate::hittable::HitRecord;
use crate::material;
use crate::material::{Material, ScatterRecord};

/// a metal material
#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Color,
    // "fuzziness" of the metal
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.min(1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = material::reflect(&r_in.direction().unit_vector(), &rec.normal);
        // set scattered to be fuzzy metallic
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some(ScatterRecord::new(self.albedo, scattered))
        } else {
            None
        }
    }
}
