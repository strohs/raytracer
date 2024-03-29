use crate::common::Ray;
use crate::hittable::{Aabb, HitRecord, Hittable};
use std::sync::Arc;

/// FlipFace is a "wrapper" struct that wraps another hittable and "flips" its front face
/// outward normal vector
#[derive(Debug)]
pub struct FlipFace {
    ptr: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn from(other: Arc<dyn Hittable>) -> Self {
        Self { ptr: other }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut hit_rec) = self.ptr.hit(r, t_min, t_max) {
            hit_rec.front_face = !hit_rec.front_face;
            Some(hit_rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.ptr.bounding_box(t0, t1)
    }
}
