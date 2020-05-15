use crate::common::{Color, Ray};
use crate::material::{Material, ScatterRecord, reflect};
use crate::common::hittable::HitRecord;

/// a metal material
#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some( ScatterRecord::new(self.albedo, scattered) )
        } else {
            None
        }
    }
}