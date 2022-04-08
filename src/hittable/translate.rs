use crate::common::{Ray, Vec3};
use crate::hittable::{Aabb, HitRecord, Hittable};
use std::sync::Arc;

/// Holds the details of a `Hittable`s translation.
/// Translation means "moving" a Hittable from some its initial location to a new
/// location in the world.
#[derive(Debug)]
pub struct Translate {
    // points to the hittable being translated
    ptr: Arc<dyn Hittable>,
    // offset from hittable's current location
    offset: Vec3,
}

impl Translate {
    /// Returns a new `Translate` hittable from the given hittable pointer: `p`.
    /// `displacement` is the amount to offset by, **not** the new location.
    pub fn from(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        match self.ptr.hit(&moved_r, t_min, t_max) {
            Some(mut rec) => {
                rec.p += self.offset;
                rec.set_face_normal(&moved_r, &rec.normal.clone());
                Some(rec)
            }
            _ => None,
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.ptr
            .bounding_box(t0, t1)
            .map(|bbox| Aabb::new(bbox.min() + self.offset, bbox.max() + self.offset))
    }
}
