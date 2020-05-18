use crate::common::{Point3, Vec3, Ray};
use crate::common;

/// a positionable camera
#[allow(dead_code)]
pub struct Camera {
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {

    /// create a new `Camera` that originates from the `look_from` point, and points at the
    /// given *look_at* point. `vup` is a Vec3 describing the camera's *up* direction, and will
    /// allows you to rotate the camera. `vfov` is the *vertical field of view* given in **degrees**.
    /// `aspect_ratio` is the desired *aspect ratio* for the rendered scene
    pub fn new(look_from: Point3,
               look_at: Point3,
               vup: Vec3,
               vfov: f64,
               aspect_ratio: f64,
               aperture: f64,
               focus_dist: f64) -> Self
    {
        let (vp_width, vp_height) = Camera::viewport_width_height(vfov, aspect_ratio);
        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let horizontal = focus_dist * vp_width * u;
        let vertical = focus_dist * vp_height * v;
        let lower_left_corner = look_from
            - horizontal / 2.0
            - vertical / 2.0
            - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Self {
            vfov,
            aspect_ratio,
            origin: look_from,
            aperture,
            focus_dist,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }


    /// return a `Ray` that originates from this camera's origin, with a direction towards
    /// the given `u, v` offsets
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let direction = self.lower_left_corner
            + (s * self.horizontal)
            + (t * self.vertical)
            - self.origin - offset;

        Ray::new(self.origin + offset, direction)
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