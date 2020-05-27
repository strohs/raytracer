use std::sync::Arc;

use crate::common::Ray;

use super::{HitRecord, Hittable};
use crate::hittable::Aabb;

/// a list of all Hittable objects in the ray tracer's "world" (a.k.a scene)
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            objects: vec![]
        }
    }
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
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }


    pub fn objects(&mut self) -> &mut Vec<Arc<dyn Hittable>> {
        &mut self.objects
    }
}

impl Hittable for HittableList {

    /// iterate through this hittable list to determine if a `Ray` has hit some
    /// object in the world. If an object was hit, `Some(HitRecord)` is returned
    /// containing details of the **closest hit**. If no object was hit by the ray,
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

    /// Returns a bounding box for the entire list of objects
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        // this will compute a surrounding bounding box for all Hittables that return a AABB,
        // AABB's that return None are filtered out
        let output_box = self.objects
            .iter()
            .map(|hittable| hittable.bounding_box(t0, t1))
            .filter(|aabb| aabb.is_some())
            .fold(Aabb::default(), |acc, aabb| {
                Aabb::surrounding_box(&acc, &aabb.unwrap())
            });

        // NOTE this is the original algorithm taken from the book, not sure why they immediately
        // return when they hit the first Hittable that doesn't return a bounding box
        // let mut output_box = Aabb::default();
        // let mut first_box = false;
        //
        // for obj in self.objects.iter() {
        //     if let Some(temp_box) = &obj.bounding_box(t0, t1) {
        //       if first_box {
        //           output_box = *temp_box;
        //           first_box = false;
        //       } else {
        //           output_box = Aabb::surrounding_box(&output_box, temp_box);
        //       }
        //     } else {
        //         return None;
        //     }
        // }
        Some(output_box)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::material::{Lambertian, Material};
    use crate::common::{Color, Point3};
    use crate::hittable::{Sphere, HittableList, Hittable};

    #[test]
    fn should_return_a_surrounding_bounding_box_with_min0_max3() {
        let material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let sphere1 = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, Arc::clone(&material));
        let sphere2 = Sphere::new(Point3::new(2.0, 2.0, 2.0), 1.0, Arc::clone(&material));
        let mut hit_list = HittableList::new();
        hit_list.add(Arc::new(sphere1));
        hit_list.add(Arc::new(sphere2));

        //t0 and t1 are ignored for regular spheres
        let surrounding_bb = hit_list.bounding_box(0.0, 1.0);
        assert_eq!(surrounding_bb.unwrap().min(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(surrounding_bb.unwrap().max(), Point3::new(3.0, 3.0, 3.0));
    }
}