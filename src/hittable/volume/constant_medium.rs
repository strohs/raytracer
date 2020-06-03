use std::sync::Arc;
use rand::Rng;
use crate::hittable::{Hittable, HitRecord, Aabb};
use crate::material::{Material, Isotropic};
use crate::texture::Texture;
use crate::common::{Ray, Vec3};

/// ConstantMedium models a volume of constant density, like smoke, fog. or mist.
/// A `Ray` that hits it can either scatter inside the volume or go all the way through it.
/// More thin transparent volumes, like a light fog, are more likely to have rays travel
/// through it. How far the ray has to travel through the volume will also determine how likely
/// it is for it to make it through.
///
/// The probability that a Ray will scatter in any small distance ΔL is:
///    `probability = C ⋅ ΔL`
/// where `C` is proportional to the optical density of the volume
///
#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {

    /// Returns a new `ConstantMedium` from the given boundary `b`, density `d`, and
    /// texture `a`
    pub fn from(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
        let phase_function: Arc<dyn Material> = Arc::new(Isotropic::from(a));
        let neg_inv_density = -1.0 / d;

        Self {
            boundary: b,
            phase_function,
            neg_inv_density,
        }
    }
}




impl Hittable for ConstantMedium {

    /// Returns `Some(HitRecord)` if the ray `r` hits this constant medium. This hit function
    /// assumes the boundary shape is **convex**. It will not work for shapes like toruses or
    /// shapes that contain voids.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // used temporarily enable debugging
        const ENABLE_DEBUG: bool = false;
        let debugging: bool = ENABLE_DEBUG && rand::thread_rng().gen::<f64>() < 0.00001;

        let mut rec1 =
            if let Some(hit_rec) = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY) {
                hit_rec
            } else {
                return None;
            };

        let mut rec2 =
            if let Some(hit_rec) = self.boundary.hit(r, rec1.t + 0.00001, f64::INFINITY) {
                hit_rec
            } else {
                return None;
            };

        if debugging { println!("nt0={:?} t1={:?}", &rec1.t, &rec2.t) }

        // need to make sure hit detection works for ray origins inside the volume
        if rec1.t < t_min { rec1.t = t_min; }
        if rec2.t > t_max { rec2.t = t_max; }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 { rec1.t = 0.0; }

        let ray_length = r.direction().length();
        let distance_inside_boudary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::thread_rng().gen::<f64>().ln();

        if hit_distance > distance_inside_boudary {
            return None;
        } else {
            let t = rec1.t + hit_distance / ray_length;
            let p = r.at(t);
            let normal = Vec3::new(1.0, 0.0, 0.0);
            let mat_ptr = Arc::clone(&self.phase_function);
            let hit_rec = HitRecord::new(p, normal, mat_ptr, t, rec1.u, rec1.v, true);
            if debugging { println!("{:?} {:?} {:?}", hit_distance, t, p); }
            Some(hit_rec)
        }

    }

    /// Returns the bounding box of this volume's `boundary`
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(t0, t1)
    }
}