use crate::common::{Point3, Vec3, Ray};

/// a simple axis-aligned camera
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {

    /// create a default camera with `origin at (0,0,0)`, `lower left corner at (-2,-1,-1)`,
    /// `horizontal (4,0,0)` and `vertical (0,2,0)`
    fn default() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            lower_left_corner: Point3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}

impl Camera {
    pub fn new(origin: Point3,
               lower_left_corner: Point3,
               horizontal: Vec3,
               vertical: Vec3) -> Self
    {
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    /// return a `Ray` that originates from this camera's origin, with a direction towards
    /// the given `u, v` offsets
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner
            + u * self.horizontal
            + v * self.vertical
            - self.origin;
        Ray::new(self.origin, direction)
    }
}