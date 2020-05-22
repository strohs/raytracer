use crate::common::{Point3, Ray};

/// Axis Aligned Bounding Box that surrounds a `Hittable`
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    min: Point3,
    max: Point3,
}

impl Default for Aabb {

    /// Returns an `Aabb` with `min` set to `INFINITY` and `max` set to `NEG_INFINITY`
    fn default() -> Self {
        let min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        Self {
            min, max
        }
    }
}

impl Aabb {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            min: a,
            max: b,
        }
    }

    /// Returns the minimum bound of this bounding box
    pub fn min(&self) -> Point3 {
        self.min
    }

    /// Returns the maximum bound of this bounding box
    pub fn max(&self) -> Point3 {
        self.max
    }

    /// Returns `Some(tmin, tmax)` if this bounding box was hit by the Ray `r`, else `None`.
    /// `tmin,tmax` are the positions on the Ray that "intersected" the bounding box.
    /// This hit function was developed by Andrew Kensler at Pixar
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<(f64, f64)> {
        let mut tmin = tmin;
        let mut tmax = tmax;

        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return None;
            }
        }

        Some((tmin, tmax))
    }

    /// Returns an axis-aligned bounding box, that surrounds `box0` **and** `box1`
    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Self {
        let small: Point3 = Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()));

        let big: Point3 = Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()));

        Aabb::new(small, big)
    }
}