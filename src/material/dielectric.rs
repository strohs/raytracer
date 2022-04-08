use crate::common::{Color, Ray};
use crate::hittable::HitRecord;
use crate::material;
use crate::material::{Material, ScatterRecord};
use rand::{thread_rng, Rng};

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
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = material::schlick(cos_theta, etai_over_etat);

        let scattered_ray =
            if etai_over_etat * sin_theta > 1.0 || thread_rng().gen::<f64>() < reflect_prob {
                // ray is always reflected OR ray had a chance to reflect
                let reflected = material::reflect(&unit_direction, &rec.normal);
                Ray::new(rec.p, reflected, r_in.time())
            } else {
                // ray is always refracted
                let refracted = material::refract(&unit_direction, &rec.normal, etai_over_etat);
                Ray::new(rec.p, refracted, r_in.time())
            };
        Some(ScatterRecord::new(attenuation, scattered_ray))
    }
}
