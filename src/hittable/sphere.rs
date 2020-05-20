use std::sync::Arc;
use crate::common::{Point3, Ray};
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};


/// a 3D sphere "object" with a `center` and `radius`
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {

    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self { center, radius, mat_ptr }
    }

    /// convenience constructor to create a Sphere from x,y,z coordinates and a radius
    pub fn from_coords(cx: f64, cy: f64, cz: f64,
                       radius: f64,
                       mat_ptr: Arc<dyn Material>) -> Self
    {
        Self {
            center: Point3::new(cx, cy, cz),
            radius,
            mat_ptr,
        }
    }


    pub fn center(&self) -> Point3 { self.center }

    pub fn radius(&self) -> f64 { self.radius }

}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        // convenience closure that builds a new HitRecord based on the Ray
        let build_hit_record = |t: f64| -> HitRecord {
            let hit_point = r.at(t);
            let outward_normal = (hit_point - self.center) / self.radius;
            HitRecord::with_face_normal(
                r,
                hit_point,
                &outward_normal,
                Arc::clone(&self.mat_ptr),
                t)
        };

        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // if the Ray hit some point on this Sphere
        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);
            let t_temp = (-half_b - root) / a;
            if t_temp < t_max && t_temp > t_min {
                return Some(build_hit_record(t_temp));
            }
            let t_temp = (-half_b + root) / a;
            if t_temp < t_max && t_temp > t_min {
                return Some(build_hit_record(t_temp));
            }
        }
        // ray did not hit this Sphere
        None
    }
}