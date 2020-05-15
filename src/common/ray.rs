use crate::common::{Vec3, Point3};

/// a three dimensional Ray consisting of an origin point and a direction ['Vec3'],
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {

    /// construct a new Ray with the given origin and direction
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    /// return a copy of this Ray's origin field
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// return a copy of this Ray's direction field
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// return the point, on this Ray, **at** the "ray parameter" `t`
    /// P(t) = A + tb
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }


}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::common::{Point3, Vec3};

    #[test]
    fn ray_default() {
        let ray = Ray::default();
        assert_eq!(ray.origin(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.direction(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn ray_origin() {
        let ray = Ray::new(
            Point3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0)
        );
        assert_eq!(ray.origin(), Point3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn ray_direction() {
        let ray = Ray::new(
            Point3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0)
        );
        assert_eq!(ray.direction(), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn ray_at() {
        let t = 2.0;
        let ray = Ray::new(
            Point3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0)
        );
        let point_at = ray.at(t);
        assert_eq!(point_at, Point3::new(9.0, 12.0, 15.0));
    }
}