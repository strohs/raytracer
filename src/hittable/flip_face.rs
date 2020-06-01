use crate::hittable::{Hittable, HitRecord, Aabb};
use std::sync::Arc;
use crate::common::Ray;

/// FlipFace is a "adapter" struct that takes another hittable and "flips" it's front face
/// outward normal
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
        return if let Some(mut hit_rec) = self.ptr.hit(&r, t_min, t_max) {
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