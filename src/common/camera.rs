use crate::common::{Point3, Vec3, Ray};
use crate::common;

/// a positionable camera
#[allow(dead_code)]
pub struct Camera {
    vfov: f64,
    aspect_ratio: f64,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {

    /// create a new `Camera` that "looks from" the given point, towards the given *look_at* point.
    /// `vup` is the camera's *up* direction, which allows you to rotate the camera by specifying
    /// which way is up. `vfov` is the *vertical field of view* in **degrees**. `aspect_ratio` is
    /// the desired *aspect ratio*
    pub fn new(look_from: Point3,
               look_at: Point3,
               vup: Vec3,
               vfov: f64,
               aspect_ratio: f64) -> Self
    {
        let (vp_width, vp_height) = Camera::viewport_width_height(vfov, aspect_ratio);
        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let horizontal = vp_width * u;
        let vertical = vp_height * v;
        let lower_left_corner = look_from
            - horizontal / 2.0
            - vertical / 2.0
            - w;

        Self {
            vfov,
            aspect_ratio,
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }


    /// return a `Ray` that originates from this camera's origin, with a direction towards
    /// the given `u, v` offsets
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner
            + (u * self.horizontal)
            + (v * self.vertical) - self.origin;
        Ray::new(self.origin, direction)
    }

    /// Computes the viewport width and height given a vertical field of view **in degrees**
    /// and an aspect ratio. Returns a tuple of `(viewport_width, viewport_height)`
    fn viewport_width_height(vfov: f64, aspect_ratio: f64) -> (f64, f64) {
        let theta = common::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let vp_height = 2.0 * h;
        let vp_width = aspect_ratio * vp_height;
        (vp_width, vp_height)
    }
}