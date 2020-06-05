use crate::common::{Point3, Vec3, Camera};
use crate::common;

/// A builder for the raytracer's `Camera`.
/// Supply all the fields and the call the `build()` function to return a new Camera
#[derive(Default, Debug, Copy, Clone)]
pub struct CameraBuilder {
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
    open_time: f64,
    close_time: f64,
}

impl CameraBuilder {

    pub fn new() -> Self {
        CameraBuilder::default()
    }

    /// Sets the camera's
    pub fn look_from(&mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        *self
    }

    pub fn look_at(&mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        *self
    }

    pub fn up_direction(&mut self, up_direction: Vec3) -> Self {
        self.vup = up_direction;
        *self
    }

    pub fn vertical_field_of_view(&mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        *self
    }

    pub fn aspect_ratio(&mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        *self
    }

    pub fn aperture(&mut self, aperture: f64) -> Self {
        self.aperture = aperture;
        *self
    }

    pub fn focus_distance(&mut self, focus_distance: f64) -> Self {
        self.focus_dist = focus_distance;
        *self
    }

    pub fn open_close_time(&mut self, open_time: f64, close_time: f64) -> Self {
        self.open_time = open_time;
        self.close_time = close_time;
        *self
    }

    /// builds and returns a new `Camera` struct
    pub fn build(&mut self) -> Camera {
        let w = (self.look_from - self.look_at).unit_vector();
        let u = self.vup.cross(w).unit_vector();
        let v = w.cross(u);

        let (vp_width, vp_height) = CameraBuilder::viewport_width_height(
            self.vfov,
            self.aspect_ratio);
        let horizontal = self.focus_dist * vp_width * u;
        let vertical = self.focus_dist * vp_height * v;
        let lower_left_corner = self.look_from
            - horizontal / 2.0
            - vertical / 2.0
            - self.focus_dist * w;
        let lens_radius = self.aperture / 2.0;

        Camera {
            look_from: self.look_from,
            open_time: self.open_time,
            close_time: self.close_time,
            lens_radius,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
        }
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