use std::sync::Arc;
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord, Aabb};
use crate::common::{Ray, Point3, Vec3};

/// a 2D, `Hittable` rectangle, that's aligned on the **xy plane**
#[derive(Debug)]
pub struct XYRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64
}

impl XYRect {

    /// Returns an axis-aligned rectangle from the given coordinates and material
    pub fn from(x0: f64, x1: f64, y0: f64, y1: f64, k:f64, mp: Arc<dyn Material>) -> Self {
        Self {
            x0, x1,
            y0, y1,
            k,
            mp
        }
    }
}

impl Hittable for XYRect {

    /// Returns `Some(HitRecord)` if the given Ray `r` intersects this Rectangle, else `None`.
    /// `t0,t1` are the time intervals of the ray
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(
            HitRecord::with_face_normal(
                r,
                r.at(t),
                &Vec3::new(0.0, 0.0, 1.0),
                Arc::clone(&self.mp),
                t,
                (x - self.x0) / (self.x1 - self.x0),
                (y - self.y0) / (self.y1 - self.y0)))
    }

    /// Returns a axis-aligned bounding box for this rectangle
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        // The bounding box will have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        let bbox = Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.001),
            Point3::new(self.x1, self.y1, self.k + 0.001));
        Some(bbox)
    }
}




/// a 2D, `Hittable` rectangle, that's aligned on the **xz plane**
#[derive(Debug)]
pub struct XZRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64
}

impl XZRect {

    /// Returns an axis-aligned rectangle from the given coordinates and material
    pub fn from(x0: f64, x1: f64, z0: f64, z1: f64, k:f64, mp: Arc<dyn Material>) -> Self {
        Self {
            x0, x1,
            z0, z1,
            k,
            mp
        }
    }
}

impl Hittable for XZRect {

    /// Returns `Some(HitRecord)` if the given Ray `r` intersects this Rectangle, else `None`.
    /// `t0,t1` are the time intervals of the ray
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(
            HitRecord::with_face_normal(
                r,
                r.at(t),
                &Vec3::new(0.0, 1.0, 0.0),
                Arc::clone(&self.mp),
                t,
                (x - self.x0) / (self.x1 - self.x0),
                (z - self.z0) / (self.z1 - self.z0)))
    }

    /// Returns a axis-aligned bounding box for this rectangle
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        // The bounding box will have non-zero width in each dimension, so pad the Y
        // dimension a small amount.
        let bbox = Aabb::new(
            Point3::new(self.x0, self.k - 0.001,self.z0),
            Point3::new(self.x1, self.k + 0.001,self.z1));
        Some(bbox)
    }
}




/// a 2D, `Hittable` rectangle, that's aligned on the **yz plane**
#[derive(Debug)]
pub struct YZRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64
}

impl YZRect {

    /// Returns an axis-aligned rectangle from the given coordinates and material
    pub fn from(y0: f64, y1: f64, z0: f64, z1: f64, k:f64, mp: Arc<dyn Material>) -> Self {
        Self {
            y0, y1,
            z0, z1,
            k,
            mp
        }
    }
}

impl Hittable for YZRect {

    /// Returns `Some(HitRecord)` if the given Ray `r` intersects this Rectangle, else `None`.
    /// `t0,t1` are the time intervals of the ray
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(
            HitRecord::with_face_normal(
                r,
                r.at(t),
                &Vec3::new(1.0, 0.0, 0.0),
                Arc::clone(&self.mp),
                t,
                (y - self.y0) / (self.y1 - self.y0),
                (z - self.z0) / (self.z1 - self.z0)))
    }

    /// Returns a axis-aligned bounding box for this rectangle
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        // The bounding box will have non-zero width in each dimension, so pad the Y
        // dimension a small amount.
        let bbox = Aabb::new(
            Point3::new(self.k - 0.001, self.y0, self.z0),
            Point3::new(self.k + 0.001, self.y1, self.z1));
        Some(bbox)
    }
}