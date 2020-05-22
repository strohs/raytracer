use crate::common::{Point3, Ray, Vec3};
use crate::material::Material;
use std::sync::Arc;
use crate::hittable::{Hittable, HitRecord, Aabb};

/// a sphere that has its center move linearly from `center0` at `time0` to `center1` at `time1`.
/// After that time interval, it continues on, so the times do not need to match up with the
/// camera's aperture open and close. This type of Sphere is capable of "motion blur" if
/// rendered by a camera that has a open shutter.
pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3, center1: Point3,
        time0: f64, time1: f64,
        radius: f64,
        mat_ptr: Arc<dyn Material>) -> Self
    {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }

    /// returns this moving sphere's center point at the given `time`
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0))
            * (self.center1 - self.center0)
    }

    /// Returns the radius of this Sphere
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // convenience closure that builds a new HitRecord based on the Ray
        let build_hit_record = |t: f64| -> HitRecord {
            let hit_point = r.at(t);
            let outward_normal = (hit_point - self.center(r.time())) / self.radius;
            HitRecord::with_face_normal(
                r,
                hit_point,
                &outward_normal,
                Arc::clone(&self.mat_ptr),
                t)
        };

        // this sphere center at the the Ray's time
        let oc = r.origin() - self.center(r.time());
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

    /// Returns a bounding box for this sphere.
    /// Rake the box of the sphere at t0, and the box of the sphere at t1, and compute the
    /// box of those two boxes
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(t0) - Vec3::new(self.radius(), self.radius(), self.radius()),
            self.center(t0) + Vec3::new(self.radius(), self.radius(), self.radius()));
        let box1 = Aabb::new(
            self.center(t1) - Vec3::new(self.radius(), self.radius(), self.radius()),
            self.center(t1) + Vec3::new(self.radius(), self.radius(), self.radius()));

        Some(Aabb::surrounding_box(&box0, &box1))
    }
}