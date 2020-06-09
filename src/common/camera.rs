use crate::common::{Point3, Vec3, Ray};
use rand::{Rng};

/// A positionable `Camera` with a configurable vertical field of view, aperture, focus distance,
/// and shutter open/close time.
///
/// All `Ray`s in this ray-tracer originate from the `Camera` via calls to its `get_ray(s,t)`
/// function
#[allow(dead_code)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub look_from: Point3,  // origin
    pub lens_radius: f64,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub open_time: f64,
    pub close_time: f64,
}

impl Camera {

    /// returns a `Ray` that originates from this camera's origin, with its direction pointing
    /// towards the given `s, t` offsets
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let direction = self.lower_left_corner
            + (s * self.horizontal)
            + (t * self.vertical)
            - self.look_from - offset;

        // generate a random amount of time the camera shutter was open
        let shutter_open: f64 = rand::thread_rng().gen_range(self.open_time, self.close_time);

        Ray::new(self.look_from + offset, direction, shutter_open)
    }

}