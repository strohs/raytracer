use crate::material::{Material, ScatterRecord};
use crate::common::{Ray, Color};
use crate::hittable::HitRecord;
use crate::material;
use rand::{Rng, thread_rng};

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    // refractive index of this Dielectric
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {

    /// scatter for a Dielectric material that **always** refracts
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = match rec.front_face {
            true => 1.0 / self.ref_idx,
            false => self.ref_idx,
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = material::schlick(cos_theta, etai_over_etat);

        let scattered_ray = if etai_over_etat * sin_theta > 1.0 {
            // ray is always reflected
            let reflected = material::reflect(&unit_direction, &rec.normal);
            Ray::new(rec.p, reflected)
        } else if thread_rng().gen::<f64>() < reflect_prob {
            // ray had a chance to reflect...so reflect it
            let reflected = material::reflect(&unit_direction, &rec.normal);
            Ray::new(rec.p, reflected)
        } else {
            // ray is always refracted
            let refracted = material::refract(&unit_direction, &rec.normal, etai_over_etat);
            Ray::new(rec.p, refracted)
        };
        Some(ScatterRecord::new(attenuation, scattered_ray))

    }
}