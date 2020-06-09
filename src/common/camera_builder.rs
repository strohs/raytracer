use crate::common::{Point3, Vec3, Camera};
use crate::common;

/// A builder struct for constructing a `Camera`.
/// Supply all the fields and the call the `build()` function to return a new Camera
#[derive(Default, Debug, Copy, Clone)]
pub struct CameraBuilder {
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64,
    image_width: u32,
    aperture: f64,
    focus_dist: f64,
    open_time: f64,
    close_time: f64,
}

impl CameraBuilder {

    pub fn new() -> Self {
        CameraBuilder::default()
    }


    ///////////////////////////////////////////////////////////
    //              Builder Functions BEGIN HERE

    /// Sets the camera's origin or, look from position
    pub fn look_from(&mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        *self
    }

    /// Sets the point the camera will be looking at
    pub fn look_at(&mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        *self
    }

    /// Sets the camera's *up vector*, which is similar to it's rotation
    /// about its origin
    pub fn up_direction(&mut self, up_direction: Vec3) -> Self {
        self.vup = up_direction;
        *self
    }

    /// Sets this camera's vertical field of view
    pub fn vertical_field_of_view(&mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        *self
    }

    /// Sets the aspect ratio for this camera and thus, the final rendered image
    pub fn aspect_ratio(&mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        *self
    }

    /// Sets the desired image_width for the final rendered image
    pub fn image_width(&mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        *self
    }

    /// Sets the camera's aperture, which, when used in combination with `focus distance`,
    /// can achieve a de-focus blur effect on objects beyond the focal distance.
    pub fn aperture(&mut self, aperture: f64) -> Self {
        self.aperture = aperture;
        *self
    }

    /// Sets the distance from the camera to the virtual focus plane. This can be used
    /// to achieve a depth of field effect.
    /// This is not the same as *focal length*. Anything at the focus plane will be in
    /// perfect focus
    pub fn focus_distance(&mut self, focus_distance: f64) -> Self {
        self.focus_dist = focus_distance;
        *self
    }

    /// Sets the camera lenses open and close time in order render a motion blur effect.
    /// This setting will only affect primitives that can *move*, such as `MoveableSphere`,
    /// and only if the primitive moves between the `open_time` and `closed_time`
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
            image_width: self.image_width,
            image_height: (self.image_width as f64 / self.aspect_ratio) as u32,
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

    //              Builder Functions END
    ///////////////////////////////////////////////////////////////////////////////////////

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