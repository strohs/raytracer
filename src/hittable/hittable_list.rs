use std::rc::Rc;
use super::{Hittable, HitRecord};
use crate::common::Ray;

/// a list of all Hittable objects in the ray tracer's "world" (a.k.a scene)
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// clear the list of all objects
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// add a hittable object to the hittable list
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

}

impl Hittable for HittableList {

    /// iterate through the hittable list to determine if a ray has hit some
    /// object in the world. If an object was hit, `Some(HitRecord)` is returned
    /// containing details of the closest "hit". If no object was hit by the ray,
    /// `None` is returned
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(hit_record);
            }
        }

        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            objects: vec![]
        }
    }
}